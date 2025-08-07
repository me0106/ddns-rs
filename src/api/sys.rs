use super::{Json, Query, Result, ok};
use crate::{
    api::error::ApiError,
    build,
    model::{Family, User},
    service::AppCtx,
};
use axum::{
    Extension, Router,
    routing::{get, post},
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route("/sys/info", get(sys_info))
        .route("/sys/init", post(init))
        .route("/sys/addrs", get(addrs))
        .route("/sys/log", get(log))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysInfo {
    config_path: String,
    initialized: bool,
    version: &'static str,
    commit_id: &'static str,
}

///系统信息
async fn sys_info(Extension(ctx): Extension<AppCtx>) -> Result<SysInfo> {
    let path = ctx.store.get_config_path();
    ok(SysInfo {
        config_path: path.to_string_lossy().to_string(),
        initialized: ctx.store.get_user().await.is_some(),
        version: build::PKG_VERSION,
        commit_id: build::SHORT_COMMIT,
    })
}

#[derive(Deserialize)]
pub struct Initial {
    user: User,
}
async fn init(Extension(ctx): Extension<AppCtx>, Json(initial): Json<Initial>) -> Result<()> {
    if ctx.initialized().await {
        return Err(ApiError::SystemInitialized);
    }
    ctx.store.save_user(initial.user).await?;
    info!("initialized system completed.");
    ok(())
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct NetworkCard {
    name: String,
    addrs: Vec<IpAddr>,
}

#[derive(Deserialize)]
struct Param {
    family: Family,
}
async fn addrs(Query(Param { family }): Query<Param>) -> Result<Vec<NetworkCard>> {
    let vec = local_ip_address::list_afinet_netifas()?;
    let addrs = vec
        .into_iter()
        .filter(|(_, addr)| match family {
            Family::Ipv4 => addr.is_ipv4(),
            Family::Ipv6 => addr.is_ipv6(),
        })
        .into_group_map_by(|(name, _)| name.to_string())
        .into_iter()
        .map(|(name, addrs)| NetworkCard {
            name,
            addrs: addrs.into_iter().sorted().map(|(_, addr)| addr).collect(),
        })
        .sorted()
        .collect();
    ok(addrs)
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct LogEvent {
    timestamp: i64,
    config: Option<String>,
    level: &'static str,
    module: String,
    message: String,
}
async fn log(Extension(ctx): Extension<AppCtx>) -> Result<Vec<LogEvent>> {
    let guard = ctx.log.buffer.lock().await;
    let vec = guard
        .iter()
        .rev()
        .map(|log| LogEvent {
            timestamp: log.timestamp.unix_timestamp(),
            config: log.config.clone(),
            level: log.level.as_str(),
            module: log.target.trim_start_matches("ddns_rs::").to_string(),
            message: log.message.clone(),
        })
        .collect_vec();
    ok(vec)
}
