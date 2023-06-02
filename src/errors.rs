use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub enum ApiError {
    BadRequest,
    NotFound,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
