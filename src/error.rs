use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// Client-facing errors. Log the underlying cause separately before returning one.
pub enum AppError {
    BadRequest(String),
    NotFound,
    ServiceUnavailable,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            Self::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "service unavailable".into(),
            ),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".into(),
            ),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
