use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database erorr: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Resource not found: {0}")]
    NotFound(String),
}
#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    message: String,
}
impl IntoResponse for AppError{
     fn into_response(self) -> axum::response::Response {
        let (status, data) = match &self {
            AppError::DatabaseError(e) => {
                eprintln!("Databasd error: {}", e);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: axum::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "A data error occurrred".to_string()
                })
            },
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: 404,
                    message: format!("Resource not found: {}", msg)
                })
            },
        };

        let body = Json(data);
        (status, body).into_response()
    }
}