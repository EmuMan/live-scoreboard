use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::io;

#[derive(Debug)]
pub enum AppError {
    TemplateError,
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::TemplateError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not parse template"),
            ).into_response(),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                String::from("Not found"),
            ).into_response(),
        }
    }
}

impl From<tera::Error> for AppError {
    fn from(_: tera::Error) -> Self {
        AppError::TemplateError
    }
}

impl From<io::Error> for AppError {
    fn from(_: io::Error) -> Self {
        AppError::NotFound
    }
}
