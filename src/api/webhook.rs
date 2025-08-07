use super::Result;
use crate::{
    api::{Json, error::ApiError, ok},
    model::{AddrConfig, DnsConfig, DnsState, Domain, Method, Webhook},
    service::{AppCtx, webhook},
};
use axum::{
    Extension, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use serde::Serialize;
use std::net::IpAddr;

pub fn router() -> Router<()> {
    Router::new()
        .route("/webhook/list", get(list))
        .route("/webhook", post(save))
        .route("/webhook/{name}", put(update))
        .route("/webhook/{name}", delete(remove))
        .route("/webhook/run/test", post(test))
}

async fn list(Extension(ctx): Extension<AppCtx>) -> Result<Vec<Webhook>> {
    ok(ctx.store.list_webhooks().await)
}

async fn save(Extension(ctx): Extension<AppCtx>, Json(webhook): Json<Webhook>) -> Result<()> {
    ctx.store.save_webhook(&webhook).await?;
    ok(())
}

async fn update(Extension(ctx): Extension<AppCtx>, Json(webhook): Json<Webhook>) -> Result<()> {
    if ctx.store.get_webhook(&webhook.name).await.is_some() {
        return ApiError::BadRequest(format!("duplicate name: {}", &webhook.name)).into();
    }
    ctx.store.save_webhook(&webhook).await?;
    ok(())
}

async fn remove(Extension(ctx): Extension<AppCtx>, Path(name): Path<String>) -> Result<()> {
    ctx.store.delete_webhook(&name).await?;
    ok(())
}

#[derive(Serialize)]
struct Data {
    data: String,
}

async fn test(Json(webhook): Json<Webhook>) -> Result<Data> {
    let config = DnsConfig {
        name: format!("test-webhook-{}", &webhook.name),
        domain: Domain {
            domain: "example.com".to_string(),
            subdomain: "@".to_string(),
        },
        interval: 0,
        ipv4: AddrConfig {
            enabled: true,
            method: Method::Nic {
                interface: "test".to_string(),
            },
            state: DnsState::Succeed {
                addr: IpAddr::V4([127, 0, 0, 1].into()),
                timestamp: 0,
            }
            .into(),
        }
        .into(),
        ipv6: Some(AddrConfig {
            enabled: true,
            method: Method::Api {
                endpoint: "https://baidu.com".to_string(),
            },
            state: DnsState::Failed {
                message: "test failed".to_string(),
                timestamp: 0,
            }
            .into(),
        }),
        provider: "".to_string(),
        webhook: Some(webhook.name.clone()),
    };
    let data = webhook::notify(&config, &webhook).await?;
    ok(Data { data })
}
