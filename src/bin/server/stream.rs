use actix_web;
use actix_web::error::HttpRangeError;
use actix_web::http::header;
use actix_web::http::HttpRange;
use actix_web::http::StatusCode;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;

use futures::Async;
use futures::Future;
use futures::Poll;
use futures::Stream;

use futures_cpupool::CpuFuture;
use futures_cpupool::CpuPool;

use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

/// A wrapper around io.Read + io.Seek which responds to HTTP Range requests.
///
/// Much of its implementation comes from actix_web's NamedFile. It is less
/// opinionated, works with io.Read + io.Seek, and supports compression. Caching
/// isn't implemented, because we think it should be implemented by wrapping
/// this responder.
///
/// It currently only responds to single ranges. The NamedFile implementation in
/// actix also shares this behavior. This is due to the lack of a strong
/// multipart writer in rust.
pub struct RangeStream<R>
where
    R: Read + Seek + Send + 'static,
{
    reader: R,

    /// Size of the resource. Sent to the client and used as the upper bound if
    /// no range is requested.
    size: u64,
}

impl<R> RangeStream<R>
where
    R: Read + Seek + Send + 'static,
{
    pub fn new(reader: R, size: u64) -> RangeStream<R> {
        RangeStream { reader, size }
    }
}

impl<R> Responder for RangeStream<R>
where
    R: Read + Seek + Send + 'static,
{
    type Item = HttpResponse;
    type Error = actix_web::Error;

    fn respond_to<S: 'static>(self, req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        let response = match req.range(self.size) {
            Ok(ref ranges) if ranges.len() > 0 => {
                let range = ranges[0];
                HttpResponse::build(StatusCode::PARTIAL_CONTENT)
                    .header(
                        header::CONTENT_RANGE,
                        format!(
                            "bytes {}-{}/{}",
                            range.start,
                            range.start + range.length - 1,
                            self.size
                        ),
                    )
                    .streaming(ChunkedStreamReader::new(
                        self.reader,
                        range,
                        req.cpu_pool().clone(),
                    ))
            }
            Ok(_) => HttpResponse::build(StatusCode::OK)
                .header(header::ACCEPT_RANGES, "bytes")
                .streaming(ChunkedStreamReader::new(
                    self.reader,
                    HttpRange {
                        start: 0,
                        length: self.size,
                    },
                    req.cpu_pool().clone(),
                )),
            Err(HttpRangeError::InvalidRange) => HttpResponse::RangeNotSatisfiable().finish(),
            Err(HttpRangeError::NoOverlap) => {
                HttpResponse::build(StatusCode::RANGE_NOT_SATISFIABLE)
                    .header(header::CONTENT_RANGE, format!("bytes */{}", self.size))
                    .finish()
            }
        };

        Ok(response)
    }
}

/// A stream which reads from an io.Read + io.Seek in chunks.
struct ChunkedStreamReader<R>
where
    R: Read + Seek + Send + 'static,
{
    /// Container for the stream. This is used to manually keep track of the
    /// lifetime of the reader.
    stream_option: Option<R>,

    /// Future which resolves once a chunk is read.
    future: Option<CpuFuture<(R, Bytes), io::Error>>,

    /// Pool which reading from the reader will be executed on.
    cpu_pool: CpuPool,

    /// When the offset is greater than or equal to this value, stop reading from
    /// the stream.
    upper_bound: u64,

    /// Number of bytes from the beginning of the file the next read should start
    /// from.
    offset: u64,

    /// Maximum size of each chunk. The last chunk may be shorter.
    chunk_size: u64,
}

impl<R> ChunkedStreamReader<R>
where
    R: Read + Seek + Send + 'static,
{
    fn new(stream: R, range: HttpRange, cpu_pool: CpuPool) -> ChunkedStreamReader<R> {
        ChunkedStreamReader {
            stream_option: Some(stream),
            future: None,
            cpu_pool,
            upper_bound: range.start + range.length,
            offset: range.start,
            chunk_size: u16::max_value() as u64,
        }
    }

    /// Waits for the current future chunk of bytes to resolve, then returns it. Panics if there is
    /// no future set.
    fn wait_for_future(&mut self) -> Poll<Option<Bytes>, io::Error> {
        let (stream, bytes) = try_ready!(self.future.as_mut().expect("no future set").poll());

        self.future.take();
        self.stream_option = Some(stream);
        self.offset += bytes.len() as u64;

        Ok(Async::Ready(Some(bytes)))
    }
}

impl<R> Stream for ChunkedStreamReader<R>
where
    R: Read + Seek + Send + 'static,
{
    type Item = Bytes;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Bytes>, io::Error> {
        if self.future.is_some() {
            return self.wait_for_future();
        }

        if self.offset >= self.upper_bound {
            // End of stream.
            return Ok(Async::Ready(None));
        }

        // Pick values off self to move into the closure to execute on the
        // cpu_pool.
        let upper_bound = self.upper_bound;
        let offset = self.offset;
        let chunk_size = self.chunk_size;
        let mut inner_stream = self
            .stream_option
            .take()
            .expect("stream used after end of stream");

        self.future = Some(self.cpu_pool.spawn_fn(move || {
            let chunk_size = u64::min(upper_bound - offset, chunk_size) as usize;
            let mut buffer = BytesMut::with_capacity(chunk_size);

            inner_stream.seek(SeekFrom::Start(offset))?;
            let bytes_read = inner_stream.read(unsafe { buffer.bytes_mut() })?;
            if bytes_read == 0 {
                return Err(io::ErrorKind::UnexpectedEof.into());
            }
            unsafe { buffer.advance_mut(bytes_read) };

            Ok((inner_stream, buffer.freeze()))
        }));

        self.wait_for_future()
    }
}
