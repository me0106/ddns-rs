use crate::{
    model::DnsConfig,
    service::{store::StoreService, task::spawn_ddns_updating_task},
};
use std::sync::Arc;
use tokio::task::JoinHandle;

#[derive(Default)]
pub struct TaskManager {
    handles: papaya::HashMap<String, JoinHandle<()>>,
}
pub async fn start_ddns_sync_svc(store: Arc<StoreService>) -> TaskManager {
    let manager = TaskManager::default();
    for config in store.list_dns_configs().await {
        let name = config.name.clone();
        let handle = spawn_ddns_updating_task(config, store.clone()).await;
        manager.handles.pin_owned().insert(name, handle);
    }
    manager
}

impl TaskManager {
    pub async fn create_task(
        &self,
        config: DnsConfig,
        store: Arc<StoreService>,
    ) -> anyhow::Result<()> {
        let name = config.name.clone();
        if self.handles.pin_owned().contains_key(&name) {
            anyhow::bail!("Duplicate task name: {}", &name);
        }
        let handle = spawn_ddns_updating_task(config, store).await;
        self.handles.pin_owned().insert(name, handle);
        Ok(())
    }

    pub async fn delete_task(&self, name: &str) {
        let guard = self.handles.pin_owned();
        if let Some(handle) = guard.remove(name) {
            handle.abort();
        }
    }
}
