use crate::{api::error::ApiError, service::AppCtx};
use axum::{
    extract::{OriginalUri, Request},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::Engine;

const PUBLIC_API: &[&str] = &["/api/user/login", "/api/sys/info", "/api/sys/init"];

pub async fn auth_layer(req: Request, next: Next) -> Response {
    let Some(OriginalUri(uri)) = req.extensions().get::<OriginalUri>() else {
        return ApiError::InternalError.into_response();
    };
    if PUBLIC_API.contains(&uri.path()) {
        return next.run(req).await;
    }
    let Some(authorization) = req.headers().get(AUTHORIZATION) else {
        return ApiError::InvalidCredential("authorization required.").into_response();
    };
    let Ok(authorization) = authorization.to_str() else {
        return ApiError::InvalidCredential("invalid authorization").into_response();
    };
    let Some((bearer, token)) = authorization.split_at_checked("Bearer".len()) else {
        return ApiError::InvalidCredential("missing bearer token").into_response();
    };
    if !bearer.eq_ignore_ascii_case("Bearer") {
        return ApiError::InvalidCredential("missing Bearer token").into_response();
    }
    let Some(ctx) = req.extensions().get::<AppCtx>() else {
        return ApiError::InternalError.into_response();
    };
    let Ok(decoded) = base64::prelude::BASE64_STANDARD.decode(token.trim()) else {
        return ApiError::InvalidCredential("invalid authorization").into_response();
    };
    if !ctx.token.validate(&decoded[..]) {
        return ApiError::InvalidCredential("invalid authorization").into_response();
    };
    next.run(req).await
}
