use std::io;
use std::io::Read;
use std::io::Write;

use server::stream::RandomRead;

/// Wraps an io.Read in a container which implements io.Read and io.Seek, reads from the underlying
/// reader only once fulfilling reads of the same region from a cache.
pub struct CachedReader<R>
    where
        R: Read,
{
    /// Underlying reader where reads are directed to
    inner: R,

    /// Where reads to the underlying reader are cached and used to fulfill future reads of the
    /// wrapping reader.
    cached: Vec<u8>,

    /// Index where the next read of the inner reader will write to in the cache. This starts at 0
    /// and increases as more bytes are read from the underlying reader.
    inner_read_head: u64,
}

impl<R> CachedReader<R>
    where
        R: Read,
{
    pub fn new(inner: R) -> CachedReader<R> {
        CachedReader {
            inner,
            cached: Vec::new(),
            inner_read_head: 0,
        }
    }

    /// Whether or not the bytes for the index is in the cache.
    fn has_cached(&self, index: u64) -> bool {
        index < self.inner_read_head
    }

    /// Calls read on the underlying reader, adding more bytes to the cache. How much is read is up
    /// to the underlying reader.
    fn read_into_cache(&mut self) -> io::Result<()> {
        let bytes_read = self
            .inner
            .read(&mut self.cached[(self.inner_read_head as usize)..])?;
        self.inner_read_head += bytes_read as u64;

        Ok(())
    }
}

impl<R> RandomRead for CachedReader<R>
    where
        R: Read,
{
    fn read(&mut self, read_from_index: u64, mut buf: &mut [u8]) -> io::Result<usize> {
        if !self.has_cached(read_from_index) {
            self.read_into_cache()?;
        }

        let available_bytes: &[u8] =
            &self.cached[(read_from_index as usize)..(self.inner_read_head as usize)];
        buf.write(available_bytes)?;

        Ok(available_bytes.len())
    }
}
