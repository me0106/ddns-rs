use super::{Json, Result, ok};
use crate::{api::error::ApiError, model::User, service::AppCtx};
use axum::{Extension, Router, routing::post};
use base64::Engine;
use serde::{Deserialize, Serialize};
use tracing::info;

pub fn router() -> Router<()> {
    Router::new()
        .route("/user/login", post(login))
        .route("/user/update", post(update))
}

#[derive(Serialize)]
pub struct AccessToken {
    pub token: String,
}
/// 登录
async fn login(
    Extension(ctx): Extension<AppCtx>,
    Json(User { username, password }): Json<User>,
) -> Result<AccessToken> {
    let Some(user) = ctx.store.get_user().await else {
        return Err(ApiError::SystemNotInitialize);
    };
    if user.username != username || !bcrypt::verify(password, &user.password)? {
        return Err(ApiError::BadRequest("invalid username or password".into()));
    }
    let token = ctx.token.generate_token().await;
    let token = base64::prelude::BASE64_STANDARD.encode(&token);
    info!("user [{username}] login successfully.");
    ok(AccessToken { token })
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub password: String,
    pub user: User,
}

async fn update(Extension(ctx): Extension<AppCtx>, Json(update): Json<UpdateUser>) -> Result<()> {
    //如果没有初始化
    let Some(exist) = ctx.store.get_user().await else {
        return Err(ApiError::SystemNotInitialize);
    };
    let verified = bcrypt::verify(&update.password, &exist.password)?;
    if verified {
        ctx.store.save_user(update.user).await?;
        return ok(());
    }
    Err(ApiError::InvalidCredential("invalid password"))
}
