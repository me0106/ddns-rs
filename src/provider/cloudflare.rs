use crate::provider::{DnsAddr, Domain, cloudflare::client::Cloudflare};
use serde::{Deserialize, Serialize};

mod client;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    api_key: String,
}

pub(super) async fn update(config: &Config, domain: Domain, addr: DnsAddr) -> anyhow::Result<()> {
    let client = Cloudflare::new(config)?;
    let zone = client
        .query_zone(&domain)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("cannot find zone for {}", domain.domain))?;
    let records = client.query_records(&zone).await?;
    if records.is_empty() {
        return client.create_records(&domain, addr, &zone).await;
    }
    client.update_records(addr, &zone, records).await
}
