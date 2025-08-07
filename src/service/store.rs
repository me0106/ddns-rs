use crate::model::{DdnsConfig, DnsConfig, Provider, User, Webhook};
use std::{path::PathBuf, sync::Arc};
use tokio::{
    fs::{File, create_dir_all, read_to_string, try_exists},
    io::AsyncWriteExt,
    sync::RwLock,
};
use tracing::{error, info};

pub struct StoreService {
    file: PathBuf,
    config: Arc<RwLock<DdnsConfig>>,
}

impl StoreService {
    pub async fn new(path: PathBuf) -> anyhow::Result<Self> {
        let exist = try_exists(&path).await?;
        if exist {
            info!("load config from {}", path.to_string_lossy());
            let config = read_to_string(&path).await?;
            let config = serde_json::from_str(&config)?;
            return Ok(Self {
                file: path,
                config: Arc::new(RwLock::new(config)),
            });
        }
        info!("create initial config at {}", path.to_string_lossy());
        let Some(parent) = path.parent() else {
            error!("parent path does not exist");
            anyhow::bail!("parent path does not exist");
        };
        if !parent.exists() {
            create_dir_all(parent).await?;
        }
        let config = DdnsConfig {
            listen: "0.0.0.0:6789".into(),
            user: None,
            ddns: vec![],
            provider: vec![],
            webhook: vec![],
        };
        let mut file = File::options().create(true).write(true).open(&path).await?;
        let value = serde_json::to_string_pretty(&config)?;
        file.write_all(value.as_bytes()).await?;
        file.flush().await?;
        info!("initial config write complete");
        Ok(Self {
            file: path,
            config: Arc::new(RwLock::new(config)),
        })
    }

    pub async fn get_listen(&self) -> String {
        self.config.read().await.listen.clone()
    }

    pub fn get_config_path(&self) -> &PathBuf {
        &self.file
    }

    pub async fn get_user(&self) -> Option<User> {
        self.config.read().await.user.clone()
    }

    pub async fn save_user(&self, user: User) -> anyhow::Result<()> {
        let mut config = self.config.write().await;
        let password = bcrypt::hash(user.password.as_bytes(), 4)?;
        let user = User { password, ..user };
        config.user = Some(user);
        flush(&self.file, &config).await?;
        Ok(())
    }

    pub async fn list_dns_configs(&self) -> Vec<DnsConfig> {
        self.config.read().await.ddns.clone()
    }

    pub async fn get_dns_config(&self, name: &str) -> Option<DnsConfig> {
        let guard = self.config.read().await;
        guard.ddns.iter().find(|c| c.name == name).cloned()
    }

    pub async fn save_dns_config(&self, config: &DnsConfig) -> anyhow::Result<()> {
        let mut guard = self.config.write().await;
        guard.ddns.retain(|c| c.name != config.name);
        guard.ddns.push(config.clone());
        flush(&self.file, &guard).await?;
        Ok(())
    }

    pub async fn delete_dns_config(&self, name: &str) -> anyhow::Result<()> {
        let mut guard = self.config.write().await;
        guard.ddns.retain(|c| c.name != name);
        flush(&self.file, &guard).await?;
        Ok(())
    }

    pub async fn get_webhook(&self, name: &str) -> Option<Webhook> {
        let guard = self.config.read().await;
        guard.webhook.iter().find(|c| c.name == name).cloned()
    }

    pub async fn save_webhook(&self, webhook: &Webhook) -> anyhow::Result<()> {
        let mut guard = self.config.write().await;
        guard.webhook.retain(|c| c.name != webhook.name);
        guard.webhook.push(webhook.clone());
        flush(&self.file, &guard).await?;
        Ok(())
    }

    pub async fn delete_webhook(&self, name: &str) -> anyhow::Result<()> {
        let mut guard = self.config.write().await;
        guard.webhook.retain(|c| c.name != name);
        flush(&self.file, &guard).await?;
        Ok(())
    }

    pub async fn list_webhooks(&self) -> Vec<Webhook> {
        self.config.read().await.webhook.clone()
    }

    pub async fn list_dns_providers(&self) -> Vec<Provider> {
        self.config.read().await.provider.clone()
    }

    pub async fn get_dns_provider(&self, name: &str) -> Option<Provider> {
        let guard = self.config.read().await;
        guard.provider.iter().find(|p| p.name == name).cloned()
    }

    pub async fn save_dns_provider(&self, provider: &Provider) -> anyhow::Result<()> {
        let mut config = self.config.write().await;
        config.provider.retain(|p| p.name != provider.name);
        config.provider.push(provider.clone());
        flush(&self.file, &config).await?;
        Ok(())
    }
    pub async fn delete_dns_provider(&self, name: &str) -> anyhow::Result<()> {
        let mut config = self.config.write().await;
        config.provider.retain(|p| p.name != name);
        flush(&self.file, &config).await?;
        Ok(())
    }
}

async fn flush(path: &PathBuf, config: &DdnsConfig) -> anyhow::Result<()> {
    tokio::fs::write(path, serde_json::to_string_pretty(config)?).await?;
    Ok(())
}
