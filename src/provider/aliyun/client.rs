use crate::{
    provider::{
        DnsAddr, Domain,
        aliyun::{
            Config,
            client::{
                schema::{AliyunError, DnsRecord, RecordId},
                signature::create_signature,
            },
        },
    },
    treemap,
};
use itertools::Itertools;
use percent_encoding::{AsciiSet, percent_encode};
use rand::RngCore;
use reqwest::{Client, header, header::AUTHORIZATION};
use serde::{Deserialize, de::DeserializeOwned};
use std::{borrow::Cow, collections::BTreeMap};
use time::{UtcDateTime, format_description::well_known::Rfc3339};

#[path = "signature.rs"]
mod signature;

#[path = "schema.rs"]
mod schema;
const VERSION: &str = "2015-01-09";

const HOST: &str = "alidns.aliyuncs.com";

const ENDPOINT: &str = "https://alidns.aliyuncs.com";

pub(super) struct Aliyun<'a> {
    config: &'a Config,
    client: Client,
}

impl<'a> Aliyun<'a> {
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
    ) -> anyhow::Result<Vec<DnsRecord>> {
        let fqdn = format!("{}.{}", &domain.subdomain, &domain.domain);
        let query = treemap! {
            "DomainName" => &*domain.domain,
            "SubDomain" => &fqdn,
            "Type" => addr.dns_type,
            "PageSize" => "500"
        };
        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "DomainRecords")]
            records: Records,
        }
        #[derive(Deserialize)]
        struct Records {
            #[serde(rename = "Record")]
            record: Vec<DnsRecord>,
        }
        let records = self
            .send::<Response>("DescribeSubDomainRecords", query)
            .await?
            .records
            .record;
        Ok(records)
    }

    pub async fn update_record(&self, addr: DnsAddr, record: &DnsRecord) -> anyhow::Result<String> {
        let value = addr.to_string();
        let query = treemap! {
            "RecordId" => &*record.record_id,
            "RR" => &record.rr,
            "Type" => addr.dns_type,
            "Value" => &value,
        };
        let id = self.send::<RecordId>("UpdateDomainRecord", query).await?;
        Ok(id.id)
    }

    pub async fn create_record(&self, domain: &Domain, addr: DnsAddr) -> anyhow::Result<String> {
        let value = addr.to_string();
        let query = treemap! {
            "DomainName" => &*domain.domain,
            "RR" => &domain.subdomain,
            "Type" => addr.dns_type,
            "Value" => &value
        };
        let id = self.send::<RecordId>("AddDomainRecord", query).await?;
        Ok(id.id)
    }

    async fn send<T: DeserializeOwned>(
        &self,
        action: &str,
        query: BTreeMap<&str, &str>,
    ) -> anyhow::Result<T> {
        const HASHED_BODY: &str =
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let query = customize_url_encode(query);
        let timestamp = UtcDateTime::now().unix_timestamp();
        let timestamp = UtcDateTime::from_unix_timestamp(timestamp)?.format(&Rfc3339)?;

        let mut random = [0u8; 32];
        rand::rng().fill_bytes(&mut random[..]);
        let random = hex::encode(&random);
        let signature = create_signature(
            &self.config,
            &timestamp,
            action,
            &query,
            &HASHED_BODY,
            &random,
        )?;
        let resp = self
            .client
            .post(format!("{ENDPOINT}?{query}"))
            .header(header::HOST, HOST)
            .header("x-acs-action", action)
            .header("x-acs-content-sha256", HASHED_BODY)
            .header("x-acs-date", timestamp)
            .header("x-acs-signature-nonce", random)
            .header("x-acs-version", VERSION)
            .header(AUTHORIZATION, signature)
            .body("")
            .send()
            .await?;
        if (400..600).contains(&resp.status().as_u16()) {
            return Err(resp.json::<AliyunError>().await?.into());
        }
        Ok(resp.json::<T>().await?)
    }
}
const SET: AsciiSet = percent_encoding::NON_ALPHANUMERIC
    .remove(b'_')
    .remove(b'-')
    .remove(b'.')
    .remove(b'~');
fn customize_url_encode(query: BTreeMap<&str, &str>) -> String {
    query
        .into_iter()
        .map(|(k, v)| {
            let k: Cow<str> = percent_encode(k.as_bytes(), &SET).collect();
            let v: Cow<str> = percent_encode(v.as_bytes(), &SET).collect();
            format!("{}={}", k, v)
        })
        .join("&")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[tokio::test]
    async fn test() {
        let config = Config {
            secret_id: dotenvy::var("ALIYUN_SECRET_ID").unwrap(),
            secret_key: dotenvy::var("ALIYUN_SECRET_KEY").unwrap(),
        };
        let aliyun = Aliyun::new(&config);
        let e = aliyun
            .query_records(
                &Domain {
                    domain: "zhouxi.me".to_string(),
                    subdomain: "@".to_string(),
                    params: Default::default(),
                },
                DnsAddr::from(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            )
            .await
            .unwrap_err();
        let e = e.downcast::<AliyunError>().unwrap();
        assert_eq!(e.code, "InvalidDomainName.NoExist")
    }
}
