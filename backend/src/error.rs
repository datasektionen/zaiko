use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    InternalServerError,
    Unauthorized,
    BadRequest,
}

impl ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(_value: sqlx::Error) -> Self {
        Error::InternalServerError
    }
}

impl From<serde_json::Error> for Error {
    fn from(_value: serde_json::Error) -> Self {
        Error::BadRequest
    }
}
