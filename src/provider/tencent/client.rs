use super::{Config, DnsAddr};
use crate::provider::Domain;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, header};
use schema::*;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use time::UtcDateTime;

#[path = "schema.rs"]
mod schema;
#[path = "signature.rs"]
mod signature;

const HOST: &str = "dnspod.tencentcloudapi.com";

const ENDPOINT: &str = "https://dnspod.tencentcloudapi.com";

const VERSION: &str = "2021-03-23";

pub struct Tencent<'a> {
    config: &'a Config,
    client: Client,
}
impl<'a> Tencent<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
    pub async fn query_records(
        &self,
        domain: &Domain,
        addr: DnsAddr,
    ) -> anyhow::Result<Vec<Record>> {
        #[derive(Deserialize)]
        pub struct Records {
            #[serde(rename = "RecordList")]
            records: Vec<Record>,
        }
        let data = DescribeRecordList {
            domain: &domain.domain,
            subdomain: &domain.subdomain,
            record_type: addr.dns_type,
        };
        let result = self.send::<Records>("DescribeRecordList", &data).await;
        let record = match result.map_err(|e| e.downcast::<TencentError>()) {
            Ok(r) => r.records,
            Err(Ok(e)) if e.error.code == "ResourceNotFound.NoDataOfRecord" => vec![],
            Err(e) => return Err(e?.into()),
        };
        Ok(record)
    }

    pub async fn update_record(
        &self,
        domain: &Domain,
        addr: DnsAddr,
        record: Record,
    ) -> anyhow::Result<u32> {
        let api = ModifyRecord {
            domain: &domain.domain,
            subdomain: &domain.subdomain,
            record_type: addr.dns_type,
            record_line: &record.line,
            value: &addr.to_string(),
            record_id: record.id,
        };
        let id = self.send::<RecordId>("ModifyRecord", &api).await?;
        Ok(*id)
    }

    pub async fn create_record(&self, domain: &Domain, addr: DnsAddr) -> anyhow::Result<u32> {
        let api = CreateRecord {
            domain: &domain.domain,
            subdomain: &domain.subdomain,
            record_type: addr.dns_type,
            record_line: "默认",
            value: &addr.to_string(),
        };
        let id = self.send::<RecordId>("CreateRecord", &api).await?;
        Ok(*id)
    }

    async fn send<R>(&self, name: &str, data: &impl Serialize) -> anyhow::Result<R>
    where
        R: DeserializeOwned,
    {
        let data = serde_json::to_string(&data)?;
        let timestamp = UtcDateTime::now().unix_timestamp();
        let authorization = signature::create_signature(&self.config, timestamp, name, &data)?;
        let data = self
            .client
            .post(ENDPOINT)
            .header(header::HOST, HOST)
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .header("X-TC-ACTION", name)
            .header("X-TC-Version", VERSION)
            .header("X-TC-Timestamp", timestamp)
            .header(AUTHORIZATION, authorization)
            .body(data)
            .send()
            .await?
            .json::<ApiResponse<R>>()
            .await?
            .response
            .content?;
        Ok(data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_send() {
        let config = Config {
            secret_id: dotenvy::var("TENCENT_SECRET_ID").unwrap(),
            secret_key: dotenvy::var("TENCENT_SECRET_KEY").unwrap(),
        };
        let api = DescribeRecordList {
            domain: "zhouxi.me",
            record_type: "A",
            subdomain: "@",
        };
        let tencent = Tencent::new(&config);
        assert!(
            tencent
                .send::<Value>("DescribeRecordList", &api)
                .await
                .unwrap()
                .as_object()
                .unwrap()
                .contains_key("RecordCountInfo")
        );
    }
}
