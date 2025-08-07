use crate::api::ApiResult;
use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::Uri,
    response::{IntoResponse, Response},
};
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("system not initialize")]
    SystemNotInitialize,

    #[error("system initialized")]
    SystemInitialized,

    #[error("json error: {0}")]
    JsonRejection(#[from] JsonRejection),

    #[error("query error: {0}")]
    QueryRejection(#[from] QueryRejection),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("path not found: {0}")]
    NotFound(Uri),

    #[error("Authentication required: {0}")]
    InvalidCredential(&'static str),

    #[error("invalid password hash")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("internal error.")]
    InternalError,

    #[error("find local addr error: {0}")]
    NicError(#[from] local_ip_address::Error),
}
impl<T> Into<super::Result<T>> for ApiError {
    fn into(self) -> super::Result<T> {
        Err(self)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::SystemNotInitialize => ApiResult::<()> {
                code: 1000,
                message: Some("system not initialized".into()),
                data: None,
            },
            ApiError::JsonRejection(r) => ApiResult::<()> {
                code: 1100,
                message: Some(Cow::from(format!(
                    "invalid request: Json error: {}",
                    r.body_text()
                ))),
                data: None,
            },
            ApiError::BadRequest(message) => ApiResult::<()> {
                code: 1100,
                message: Some(message.into()),
                data: None,
            },
            ApiError::InvalidCredential(message) => ApiResult::<()> {
                code: 1200,
                message: Some(message.into()),
                data: None,
            },
            ApiError::NotFound(uri) => ApiResult::<()> {
                code: 1300,
                message: Some(format!("path {uri} not found").into()),
                data: None,
            },

            ApiError::Bcrypt(_) => ApiResult::<()> {
                code: 1500,
                message: Some("Internal error: invalid password hash".into()),
                data: None,
            },

            ApiError::Anyhow(e) => ApiResult::<()> {
                code: 1600,
                message: Some(e.to_string().into()),
                data: None,
            },
            ApiError::InternalError => ApiResult::<()> {
                code: 1700,
                message: Some("System internal error".into()),
                data: None,
            },
            ApiError::SystemInitialized => ApiResult::<()> {
                code: 1110,
                message: Some("system already initialized".into()),
                data: None,
            },
            ApiError::NicError(e) => ApiResult::<()> {
                code: 1800,
                message: Some(e.to_string().into()),
                data: None,
            },
            ApiError::QueryRejection(e) => ApiResult::<()> {
                code: 1100,
                message: Some(e.to_string().into()),
                data: None,
            },
        }
        .into_response()
    }
}
