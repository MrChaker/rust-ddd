use super::db_error::DatabaseError;
use crate::user::errors::user_errors::UserError;

use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::UserError(error) => error.into_response(),
            ApiError::DatabaseError(error) => error.into_response(),
        }
    }
}
