use axum::{
    body::Body,
    extract::{Extension, Path},
    response::{Html, Response},
    Router,
    routing::get,
};
use std::{fs, sync::Arc};

use super::AppState;
use super::error::AppError;


pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/scoreboard", get(render_scoreboard))
        .route("/assets/*path", get(serve_asset))
        .layer(Extension(state))
}

pub async fn render_scoreboard(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    // Lock the context to safely access it
    let context = state.context.lock().unwrap();

    // Render the template
    let rendered = state
        .tera
        .render("scoreboard.html", &context)?;

    Ok(Html(rendered))
}

pub async fn serve_asset(Path(path): Path<String>) -> Result<Response, AppError> {
    if path.contains("..") {
        return Err(AppError::NotFound);
    }

    // Match path end with whether it is text or binary
    let content_type = get_content_type(path.as_str());
    let body = if content_type.starts_with("text") {
        Body::from(fs::read_to_string(format!("assets/{}", path))?)
    } else {
        Body::from(fs::read(format!("assets/{}", path))?)
    };

    let mut response = Response::new(body);
    response.headers_mut().insert("Content-Type", content_type.parse().unwrap());
    Ok(response)
}

fn get_content_type(path: &str) -> &'static str {
    match path.split('.').last() {
        Some("css") => "text/css",
        Some("js") => "text/javascript",
        Some("png") => "image/png",
        Some("jpg") => "image/jpeg",
        Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}
