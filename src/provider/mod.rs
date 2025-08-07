use crate::model::{Provider, RealProvider};
use std::{collections::BTreeMap, net::IpAddr, ops::Deref};
use url::form_urlencoded::parse;

use crate::model;
pub use aliyun::Config as AliyunConfig;
pub use cloudflare::Config as CloudflareConfig;
pub use tencent::Config as TencentConfig;
mod aliyun;
mod cloudflare;
mod digest;
mod tencent;

#[macro_export]
macro_rules! treemap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::BTreeMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

type Map = BTreeMap<String, String>;
pub(self) struct Domain {
    pub domain: String,
    pub subdomain: String,
    #[allow(dead_code)]
    pub params: Map,
}

impl From<&model::Domain> for Domain {
    fn from(domain: &model::Domain) -> Self {
        let model::Domain { domain, subdomain } = domain;
        let mut params = Map::default();
        let (domain, param) = match domain.split_once("?") {
            None => (&**domain, ""),
            Some(v) => v,
        };
        for (k, v) in parse(param.as_bytes()) {
            params.insert(k.into(), v.into());
        }
        Self {
            domain: domain.into(),
            subdomain: subdomain.into(),
            params,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(self) struct DnsAddr {
    addr: IpAddr,
    dns_type: &'static str,
}
impl Deref for DnsAddr {
    type Target = IpAddr;

    fn deref(&self) -> &Self::Target {
        &self.addr
    }
}
impl From<IpAddr> for DnsAddr {
    fn from(addr: IpAddr) -> Self {
        let dns_type = match addr {
            IpAddr::V4(_) => "A",
            IpAddr::V6(_) => "AAAA",
        };
        Self { addr, dns_type }
    }
}

pub async fn update_ddns_record(
    domain: &model::Domain,
    provider: &Provider,
    addr: IpAddr,
) -> anyhow::Result<()> {
    use RealProvider::*;
    match &provider.config {
        Tencent(config) => tencent::update(config, domain.into(), addr.into()).await,
        Cloudflare(config) => cloudflare::update(config, domain.into(), addr.into()).await,
        Aliyun(config) => aliyun::update(config, domain.into(), addr.into()).await,
    }
}
