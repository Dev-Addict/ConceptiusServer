use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum OperationalError {
    NotFound,
}

impl Display for OperationalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Not Found"),
        }
    }
}

impl Error for OperationalError {}

impl IntoResponse for OperationalError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            OperationalError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
        };

        (status_code, message).into_response()
    }
}
