use crate::provider::{AliyunConfig, CloudflareConfig, TencentConfig};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    net::IpAddr,
};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct DdnsConfig {
    pub listen: String,
    pub user: Option<User>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ddns: Vec<DnsConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<Provider>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub webhook: Vec<Webhook>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Domain {
    pub domain: String,
    pub subdomain: String,
}
impl Domain {
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.subdomain, self.domain)
    }
}
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct DnsConfig {
    pub name: String,
    #[serde(flatten)]
    pub domain: Domain,
    pub interval: u64,
    pub ipv4: Option<AddrConfig>,
    pub ipv6: Option<AddrConfig>,
    pub provider: String,
    pub webhook: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Family {
    Ipv4,
    Ipv6,
}
impl Display for Family {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Family::Ipv4 => "ipv4",
            Family::Ipv6 => "ipv6",
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
#[serde(tag = "method")]
pub enum Method {
    Api {
        #[serde(default)]
        endpoint: String,
    },
    Nic {
        #[serde(default)]
        interface: String,
    },
    Cmd {
        #[serde(default)]
        command: String,
    },
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct AddrConfig {
    pub enabled: bool,
    #[serde(flatten)]
    pub method: Method,
    pub state: Option<DnsState>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum DnsState {
    Succeed { timestamp: u64, addr: IpAddr },
    Failed { timestamp: u64, message: String },
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Provider {
    pub name: String,
    #[serde(flatten)]
    pub config: RealProvider,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum RealProvider {
    Tencent(TencentConfig),
    Cloudflare(CloudflareConfig),
    Aliyun(AliyunConfig),
}
impl RealProvider {
    pub fn ty(&self) -> &'static str {
        match self {
            Self::Tencent(_) => "tencent",
            Self::Cloudflare(_) => "cloudflare",
            Self::Aliyun(_) => "aliyun",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Webhook {
    pub name: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use crate::model::DnsConfig;
    use serde_json::json;

    #[test]
    fn test() {
        let data = json!({
            "name": "",
            "domain": "",
            "subdomain": "",
            "interval": 5,
            "ipv4": {
                "enabled": true,
                "method": "api",
                "endpoint": "",
            },
            "ipv6": {
                "enabled": true,
                "method": "api",
                "endpoint": "",
            },
            "provider": "",
            "webhook": ""
        });
        serde_json::from_value::<DnsConfig>(data).unwrap();
    }
}
