use super::{Json, Result, ok};
use crate::{
    api::{
        dns::State::{Disabled, Failure, Pending, Success},
        error::ApiError,
    },
    model::{AddrConfig, DnsConfig, DnsState, Domain},
    service::AppCtx,
};
use axum::{
    Extension, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use serde::Serialize;
use std::net::IpAddr;

pub fn router() -> Router {
    Router::new()
        .route("/dns/{name}", get(get_by_name))
        .route("/dns", post(save))
        .route("/dns/{name}", put(update))
        .route("/dns/{name}", delete(remove))
        .route("/dns/state/list", get(state))
        .route("/dns/run/{name}", put(run))
}
#[derive(Serialize)]
struct DnsStateView {
    name: String,
    #[serde(flatten)]
    domain: Domain,
    kind: Option<&'static str>,
    ipv4: State,
    ipv6: State,
}

#[derive(Serialize)]
#[serde(tag = "state")]
#[serde(rename_all = "camelCase")]
enum State {
    Success { timestamp: u64, addr: IpAddr },
    Failure { timestamp: u64, message: String },
    Pending,
    Disabled,
}
impl From<DnsState> for State {
    fn from(value: DnsState) -> Self {
        match value {
            DnsState::Succeed { timestamp, addr } => Success { timestamp, addr },
            DnsState::Failed { timestamp, message } => Failure { timestamp, message },
        }
    }
}

async fn state(Extension(AppCtx { store, .. }): Extension<AppCtx>) -> Result<Vec<DnsStateView>> {
    let configs = store.list_dns_configs().await;
    let mut views = Vec::with_capacity(configs.len());
    for config in configs {
        let DnsConfig {
            name,
            domain,
            ipv4,
            ipv6,
            provider,
            ..
        } = config;
        let ty = store.get_dns_provider(&provider).await;
        let ipv4 = construct_state(ipv4);
        let ipv6 = construct_state(ipv6);
        views.push(DnsStateView {
            name,
            domain,
            kind: ty.map(|p| p.config.ty()),
            ipv4,
            ipv6,
        });
    }
    ok(views)
}
#[inline]
fn construct_state(config: Option<AddrConfig>) -> State {
    let Some(config) = config else {
        return Disabled;
    };
    if !config.enabled {
        return Disabled;
    };
    match config.state {
        None => Pending,
        Some(state) => state.into(),
    }
}

async fn get_by_name(
    Extension(ctx): Extension<AppCtx>,
    Path(name): Path<String>,
) -> Result<Option<DnsConfig>> {
    ok(ctx.store.get_dns_config(&name).await)
}

async fn save(Extension(ctx): Extension<AppCtx>, Json(config): Json<DnsConfig>) -> Result<()> {
    if ctx.store.get_dns_config(&config.name).await.is_some() {
        return ApiError::BadRequest(format!("duplicate name: {}", &config.name)).into();
    }
    save0(ctx, config).await
}

async fn update(Extension(ctx): Extension<AppCtx>, Json(config): Json<DnsConfig>) -> Result<()> {
    save0(ctx, config).await
}

async fn remove(Extension(ctx): Extension<AppCtx>, Path(name): Path<String>) -> Result<()> {
    ctx.manager.delete_task(&name).await;
    ctx.store.delete_dns_config(&name).await?;
    ok(())
}

async fn run(Extension(ctx): Extension<AppCtx>, Path(name): Path<String>) -> Result<()> {
    let Some(config) = ctx.store.get_dns_config(&name).await else {
        return ApiError::BadRequest(format!("dns config not found: {name}",)).into();
    };
    ctx.manager.delete_task(&name).await;
    ctx.manager.create_task(config, ctx.store).await?;
    ok(())
}

async fn save0(AppCtx { store, manager, .. }: AppCtx, config: DnsConfig) -> Result<()> {
    if store.get_dns_provider(&config.provider).await.is_none() {
        return ApiError::BadRequest(format!("provider not found: {}", &config.provider)).into();
    }
    if let Some(webhook) = &config.webhook
        && store.get_webhook(webhook).await.is_none()
    {
        return ApiError::BadRequest(format!("webhook not found: {webhook}",)).into();
    }
    manager.delete_task(&config.name).await;
    store.save_dns_config(&config).await?;
    manager.create_task(config, store).await?;
    ok(())
}
