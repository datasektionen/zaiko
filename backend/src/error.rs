use std::env::VarError;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use openidconnect::{
    core::CoreErrorResponseType, reqwest, ClaimsVerificationError, ConfigurationError,
    HttpClientError, RequestTokenError, SignatureVerificationError, SigningError,
    StandardErrorResponse,
};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Display, ToResponse, ToSchema)]
pub enum Error {
    InternalServerError(String),
    Unauthorized,
    BadRequest,
}

impl ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<sqlx::types::uuid::Error> for Error {
    fn from(value: sqlx::types::uuid::Error) -> Self {
        log::error!("{}", value);
        Error::BadRequest
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("sql: {}", value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        log::error!("{}", value);
        Error::BadRequest
    }
}

impl From<ConfigurationError> for Error {
    fn from(value: ConfigurationError) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("token clinet request config error: {}", value))
    }
}

impl From<ClaimsVerificationError> for Error {
    fn from(value: ClaimsVerificationError) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("aquireing claims error: {}", value))
    }
}

impl From<SignatureVerificationError> for Error {
    fn from(value: SignatureVerificationError) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("aquireing signing algorithm: {}", value))
    }
}

impl From<SigningError> for Error {
    fn from(value: SigningError) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("checking hash error: {}", value))
    }
}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("env var error: {}", value))
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("request error: {}", value))
    }
}

impl<ER> From<RequestTokenError<HttpClientError<ER>, StandardErrorResponse<CoreErrorResponseType>>>
    for Error
where
    ER: core::error::Error + 'static,
{
    fn from(
        value: RequestTokenError<HttpClientError<ER>, StandardErrorResponse<CoreErrorResponseType>>,
    ) -> Self {
        log::error!("{}", value);
        Error::InternalServerError(format!("token request error: {}", value))
    }
}
