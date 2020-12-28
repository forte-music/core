use actix_web;
use actix_web::http::header;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use mime_guess;
use mime_guess::Mime;
use server::stream::RangeStream;
use std::fs::File;
use std::io;
use std::path::Path;

/// A wrapper around RangeStream<File> which handles setting the Content-Type header based on the
/// mime type of the file.
pub struct FileStream {
    mime: Option<Mime>,
    inner: RangeStream<File>,
}

impl FileStream {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<FileStream, io::Error> {
        let path = path.as_ref();

        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let size = metadata.len();

        let inner = RangeStream::new(file, size);
        let mime = path
            .extension()
            .map(|ext| mime_guess::from_ext(&ext.to_string_lossy()).first_or_octet_stream());

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
                .insert(header::CONTENT_TYPE, mime.to_string().parse()?);
        }

        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::FileStream;
    use actix_web::test::TestRequest;
    use actix_web::HttpResponse;
    use actix_web::Responder;
    use std::path::PathBuf;

    #[test]
    fn sets_mime_type() {
        let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file_path = manifest_path.join("src/lib.rs");

        let file_stream = FileStream::open(file_path).unwrap();

        let req = TestRequest::default().finish();
        let response: HttpResponse = file_stream.respond_to(&req).unwrap();

        let content_type = response.headers().get("Content-Type");
        assert!(!content_type.unwrap().is_empty());
    }
}
