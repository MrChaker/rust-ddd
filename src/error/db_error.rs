use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::response::api_response::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Something went wrong")]
    SomethingWentWrong(String),
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::SomethingWentWrong(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
