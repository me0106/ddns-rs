use crate::provider::{DnsAddr, Domain, cloudflare::Config};
use anyhow::anyhow;
use reqwest::{
    Client, ClientBuilder,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

const ZONE_ENDPOINT: &str = "https://api.cloudflare.com/client/v4/zones";
const DNS_ENDPOINT: &str = "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records";
const UPDATE_DNS_ENDPOINT: &str =
    "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{dns_record_id}";

pub struct Cloudflare {
    client: Client,
}
impl Cloudflare {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", &config.api_key);
        let header = HeaderValue::from_bytes(auth_value.as_bytes())?;
        headers.insert(AUTHORIZATION, header);
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(Self { client })
    }
    pub async fn query_zone(&self, domain: &Domain) -> anyhow::Result<Vec<Zone>> {
        self.client
            .get(ZONE_ENDPOINT)
            .query(&[("name", &*domain.domain), ("page", "1"), ("per_page", "50")])
            .send()
            .await?
            .json::<ApiResponse<Vec<Zone>>>()
            .await?
            .into()
    }

    pub async fn query_records(&self, zone: &Zone) -> anyhow::Result<Vec<DnsRecord>> {
        let url = DNS_ENDPOINT.replace("{zone_id}", &zone.id);
        self.client
            .get(url)
            .query(&[("name", &*zone.name), ("page", "1"), ("per_page", "50")])
            .send()
            .await?
            .json::<ApiResponse<Vec<DnsRecord>>>()
            .await?
            .into()
    }

    pub async fn update_records(
        &self,
        addr: DnsAddr,
        zone: &Zone,
        records: Vec<DnsRecord>,
    ) -> anyhow::Result<()> {
        for record in records {
            let DnsRecord {
                name,
                kind,
                proxied,
                ..
            } = record;
            let url = UPDATE_DNS_ENDPOINT
                .replace("{zone_id}", &zone.id)
                .replace("{dns_record_id}", &record.id);
            let response = self
                .client
                .patch(url)
                .json(&ModifyingDnsRecord {
                    name: &*name,
                    kind,
                    proxied,
                    content: addr.to_string(),
                })
                .send()
                .await?
                .json::<ApiResponse<()>>()
                .await?;
            Into::<anyhow::Result<()>>::into(response)?;
        }
        Ok(())
    }

    pub async fn create_records(
        &self,
        domain: &Domain,
        addr: DnsAddr,
        zone: &Zone,
    ) -> anyhow::Result<()> {
        let url = DNS_ENDPOINT.replace("{zone_id}", &zone.id);
        let resp = self
            .client
            .post(url)
            .json(&ModifyingDnsRecord {
                name: &domain.domain,
                content: addr.to_string(),
                kind: addr.dns_type.to_string(),
                proxied: false,
            })
            .send()
            .await?
            .json::<ApiResponse<()>>()
            .await?;
        Into::<anyhow::Result<()>>::into(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    #[serde(default)]
    pub errors: Vec<ApiError>,
    pub success: bool,
    #[serde(default)]
    pub result: Option<T>,
}

impl<T> Into<anyhow::Result<T>> for ApiResponse<T> {
    fn into(self) -> anyhow::Result<T> {
        if self.success {
            return self
                .result
                .ok_or_else(|| anyhow!("Api returned success. but result is none"));
        }
        if let Some(e) = &self.errors.first() {
            anyhow::bail!("Failed to call cloudflare api. code: {e:#}");
        };
        anyhow::bail!("Unknown error to request cloudflare api");
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiError {
    pub code: u16,
    pub message: String,
    #[serde(rename = "error_chain", default)]
    pub errors: Vec<ApiError>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}. ", self.code, self.message))?;
        for err in &self.errors {
            std::fmt::Display::fmt(&err, f)?;
        }
        Ok(())
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub paused: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRecord {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub content: String,
    pub proxied: bool,
    pub proxiable: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyingDnsRecord<'a> {
    pub name: &'a str,
    #[serde(rename = "type")]
    pub kind: String,
    pub content: String,
    pub proxied: bool,
}
