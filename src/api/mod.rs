use crate::{api::error::ApiError, service::AppCtx};
use axum::{
    Extension, Router,
    extract::{FromRequest, FromRequestParts, OriginalUri},
    middleware::from_fn,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::borrow::Cow;

mod dns;
mod error;
mod middleware;
mod provider;
mod sys;
mod user;
mod webhook;

pub(self) type Result<T> = std::result::Result<ApiResult<T>, ApiError>;

pub fn router(app: AppCtx) -> Router {
    let api = Router::new()
        .merge(user::router())
        .merge(sys::router())
        .merge(dns::router())
        .merge(provider::router())
        .merge(webhook::router())
        .fallback(not_found)
        .layer(from_fn(middleware::auth_layer))
        .layer(Extension(app));
    Router::new().nest("/api", api)
}

pub(self) fn ok<T: Serialize>(data: T) -> Result<T> {
    Ok(ApiResult::<T> {
        code: 0,
        message: None,
        data: data.into(),
    })
}

async fn not_found(OriginalUri(uri): OriginalUri) -> Result<()> {
    Err(ApiError::NotFound(uri))
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(error::ApiError))]
pub(self) struct Json<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(error::ApiError))]
pub(self) struct Query<T>(pub T);

////////////////// ApiResult /////////////////////////

//返回的结构
#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub(self) struct ApiResult<T = ()> {
    code: u32,
    message: Option<Cow<'static, str>>,
    data: Option<T>,
}

impl<T> IntoResponse for ApiResult<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
