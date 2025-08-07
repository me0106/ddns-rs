use crate::service::{
    dns::{TaskManager, start_ddns_sync_svc},
    log::LogService,
    store::StoreService,
    token::TokenService,
};
use std::{path::PathBuf, sync::Arc};

mod store;
mod token;

mod dns;
mod task;

pub mod log;
pub mod webhook;

#[derive(Clone)]
pub struct AppCtx {
    pub store: Arc<StoreService>,
    pub token: Arc<TokenService>,
    pub manager: Arc<TaskManager>,
    pub log: Arc<LogService>,
}

impl AppCtx {
    pub async fn new(file: PathBuf) -> anyhow::Result<Self> {
        let log = LogService::init().into();
        let store = Arc::new(StoreService::new(file).await?);
        let token = Arc::new(TokenService::new());
        token.clone().start_evict_expired_token().await;
        let manager = start_ddns_sync_svc(store.clone()).await.into();
        let ctx = Self {
            store,
            token,
            manager,
            log,
        };
        Ok(ctx)
    }

    pub async fn initialized(&self) -> bool {
        self.store.get_user().await.is_some()
    }
}
