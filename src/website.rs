use axum::{
    Router,
    http::{StatusCode, Uri, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
    routing::get,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "web/dist/"]
#[allow_missing = true]
struct Web;

const INDEX: &str = "index.html";

pub fn router() -> Router {
    Router::new().fallback(get(website))
}

async fn website(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    if path == INDEX || !path.contains(".") || path.is_empty() {
        return send_static(INDEX).await;
    }
    send_static(path).await
}

async fn send_static(path: &str) -> Response {
    let ext = match path.rsplit_once(".") {
        Some((_, ty)) => ty,
        None => "html",
    };

    let mime = match ext {
        "html" => mime::TEXT_HTML_UTF_8,
        "js" => mime::TEXT_JAVASCRIPT,
        "css" => mime::TEXT_CSS_UTF_8,
        "png" => mime::IMAGE_PNG,
        "svg" => mime::IMAGE_SVG,
        "jpg" | "jpeg" => mime::IMAGE_JPEG,
        "gif" => mime::IMAGE_GIF,
        "bmp" => mime::IMAGE_BMP,
        _ => mime::TEXT_PLAIN_UTF_8,
    };
    Web::get(&path)
        .map(|file| ([(CONTENT_TYPE, mime.essence_str())], file.data).into_response())
        .unwrap_or_else(not_found)
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
