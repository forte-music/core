use crate::server::graphql::AppState;
use crate::server::WebHandlerAppExt;
use actix_web::{http::Method, App, HttpRequest, HttpResponse};

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
            .body(content),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

/// Return the index page of the web interface
pub fn web_interface_index<S>(_req: HttpRequest<S>) -> HttpResponse {
    handle_embedded_file("index.html")
}

/// Return the requested page/file, if it exists.
pub fn web_interface<S>(req: HttpRequest<S>) -> HttpResponse {
    // Trim the preceding `/` in path
    handle_embedded_file(&req.path()[1..])
}

impl WebHandlerAppExt for App<AppState> {
    fn register_web_interface_handler(self) -> Self {
        self.route("/", Method::GET, web_interface_index).route(
            "/{_:.*}",
            Method::GET,
            web_interface,
        )
    }
}
