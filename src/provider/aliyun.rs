use crate::provider::{DnsAddr, Domain, aliyun::client::Aliyun};
use serde::{Deserialize, Serialize};
use tracing::info;

mod client;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    secret_id: String,
    secret_key: String,
}

pub async fn update(config: &Config, domain: Domain, addr: DnsAddr) -> anyhow::Result<()> {
    let client = Aliyun::new(config);
    let Some(record) = client
        .query_records(&domain, addr)
        .await?
        .into_iter()
        .next()
    else {
        let id = client.create_record(&domain, addr).await?;
        info!("Created record {} with id {}", &*addr, id);
        return Ok(());
    };
    if record.value != addr.to_string() {
        client.update_record(addr, &record).await?;
    }
    Ok(())
}
