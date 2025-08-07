use crate::provider::{DnsAddr, Domain, tencent::client::Tencent};
use serde::{Deserialize, Serialize};
use tracing::info;

mod client;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    secret_id: String,
    secret_key: String,
}

pub(super) async fn update(config: &Config, domain: Domain, addr: DnsAddr) -> anyhow::Result<()> {
    let tencent = Tencent::new(config);
    //TODO Duplicated
    let Some(record) = tencent
        .query_records(&domain, addr)
        .await?
        .into_iter()
        .next()
    else {
        let id = tencent.create_record(&domain, addr).await?;
        info!("Created Dns record {} with id {}", &*addr, id);
        return Ok(());
    };
    if record.value != addr.to_string() {
        tencent.update_record(&domain, addr, record).await?;
    }
    Ok(())
}
