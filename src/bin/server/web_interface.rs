use actix_web::body::Body;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse};
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "web/"]
struct WebAssets;

/// Get a file from the embedded web assets
fn handle_embedded_file(path: &str) -> HttpResponse {
    match WebAssets::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(path)
                    .first_or_octet_stream()
                    .to_string(),
            )
            .body(match content {
                // Handle each case separately so we avoid copying the data
                // if possible (two different From impls)
                Cow::Owned(content) => Body::from(content),
                Cow::Borrowed(content) => Body::from(content),
            }),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

/// Return the index page of the web interface
pub fn web_interface_index() -> HttpResponse {
    handle_embedded_file("index.html")
}

/// Return the requested page/file, if it exists.
pub fn web_interface(req: HttpRequest) -> HttpResponse {
    // Trim the preceding `/` in path
    handle_embedded_file(&req.path()[1..])
}

pub fn register_web_interface_handler(config: &mut ServiceConfig) {
    config
        .route("/", web::get().to(web_interface_index))
        .route("/{_:.*}", web::get().to(web_interface));
}
