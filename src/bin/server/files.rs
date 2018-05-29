use actix_web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::http::header;
use mime_guess;
use server::stream::RangeStream;
use std::fs::File;
use std::io;
use std::path::Path;

/// A wrapper around RangeStream<File> which handles setting the Content-Type header based on the
/// mime type of the file.
pub struct FileStream {
    mime: Option<String>,
    inner: RangeStream<File>,
}

impl FileStream {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<FileStream, io::Error> {
        let path = path.as_ref();

        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let size = metadata.len();

        let inner = RangeStream::new(file, size);
        let mime = path.extension()
            .map(|ext| mime_guess::get_mime_type(&ext.to_string_lossy()))
            .map(|mime| mime.to_string());

        Ok(FileStream { mime, inner })
    }
}

impl Responder for FileStream {
    type Item = HttpResponse;
    type Error = actix_web::Error;

    fn respond_to<S: 'static>(self, req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        let mut response: HttpResponse = self.inner.respond_to(req)?;
        if let Some(mime) = self.mime {
            response
                .headers_mut()
                .insert(header::CONTENT_TYPE, mime.parse()?);
        }

        Ok(response)
    }
}
