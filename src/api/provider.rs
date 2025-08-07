use crate::{
    api::{Json, Result, error::ApiError, ok},
    model::Provider,
    service::AppCtx,
};
use axum::{
    Extension, Router,
    extract::Path,
    routing::{delete, get, post, put},
};

pub fn router() -> Router<()> {
    Router::new()
        .route("/provider/list", get(list))
        .route("/provider", post(save))
        .route("/provider/{name}", put(update))
        .route("/provider/{name}", delete(remove))
}

async fn list(Extension(ctx): Extension<AppCtx>) -> Result<Vec<Provider>> {
    ok(ctx.store.list_dns_providers().await)
}

async fn save(Extension(ctx): Extension<AppCtx>, Json(provider): Json<Provider>) -> Result<()> {
    if ctx.store.get_dns_provider(&provider.name).await.is_some() {
        return ApiError::BadRequest(format!("duplicate name: {}", &provider.name)).into();
    }
    ctx.store.save_dns_provider(&provider).await?;
    ok(())
}
async fn update(Extension(ctx): Extension<AppCtx>, Json(provider): Json<Provider>) -> Result<()> {
    ctx.store.save_dns_provider(&provider).await?;
    ok(())
}

async fn remove(Extension(ctx): Extension<AppCtx>, Path(name): Path<String>) -> Result<()> {
    ctx.store.delete_dns_provider(&name).await?;
    ok(())
}
