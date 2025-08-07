use crate::{
    model::{
        AddrConfig, DnsConfig, DnsState, Domain, Family,
        Family::{Ipv4, Ipv6},
        Method, Provider,
    },
    provider::update_ddns_record,
    service::{store::StoreService, webhook},
};
use std::{net::IpAddr, str::FromStr, sync::Arc, time::Duration};
use time::UtcDateTime;
use tokio::{process::Command, task::JoinHandle, time::interval};
use tracing::{debug, error, info, instrument};

pub async fn spawn_ddns_updating_task(
    config: DnsConfig,
    store: Arc<StoreService>,
) -> JoinHandle<()> {
    tokio::spawn(task(config, store))
}
#[instrument("", skip_all, fields(config = %config.name))]
async fn task(mut config: DnsConfig, store: Arc<StoreService>) {
    let duration = Duration::from_secs(config.interval);
    let mut interval = interval(duration);
    info!("start ddns updating: interval={duration:?}",);
    loop {
        interval.tick().await;
        let DnsConfig {
            domain,
            ipv4,
            ipv6,
            provider,
            ..
        } = &mut config;
        let provider = store.get_dns_provider(provider).await;
        let mut configs = Vec::with_capacity(2);
        if let Some(ipv4) = ipv4
            && ipv4.enabled
        {
            configs.push((ipv4, Ipv4))
        }
        if let Some(ipv6) = ipv6
            && ipv6.enabled
        {
            configs.push((ipv6, Ipv6))
        }
        if configs.is_empty() {
            info!("config no valid ipv4/ipv6 definition. task terminated.",);
            return;
        }
        update(domain, provider, &mut configs).await;
        match store.save_dns_config(&config).await {
            Ok(_) => debug!("save dns config state success"),
            Err(e) => error!("save dns config state fail: {e:#}"),
        };
        notify(&config, &store).await;
    }
}

async fn update(
    domain: &Domain,
    provider: Option<Provider>,
    configs: &mut [(&mut AddrConfig, Family)],
) {
    let Some(provider) = provider else {
        return;
    };
    let timestamp = UtcDateTime::now().unix_timestamp() as _;
    for (cfg, family) in configs {
        if !cfg.enabled {
            continue;
        }
        let state = match do_update(domain, &provider, (cfg, *family)).await {
            Ok(addr) => {
                info!("update [{family}] success: {addr}");
                DnsState::Succeed { addr, timestamp }
            }
            Err(e) => {
                let message = format!("{:#}", e);
                error!("update [{family}] failure: {message}");
                DnsState::Failed { message, timestamp }
            }
        };
        cfg.state = Some(state);
    }
}

async fn notify(config: &DnsConfig, store: &StoreService) {
    if let Some(name) = &config.webhook
        && let Some(webhook) = store.get_webhook(name).await
    {
        match webhook::notify(config, &webhook).await {
            Ok(_) => info!("webhook successfully sent"),
            Err(e) => error!("webhook notification failed: {:#}", e),
        };
    }
}

async fn do_update(
    domain: &Domain,
    provider: &Provider,
    (config, family): (&AddrConfig, Family),
) -> anyhow::Result<IpAddr> {
    let addr = find_addr(config, family)
        .await?
        .ok_or_else(|| anyhow::anyhow!("cannot find valid ip address"))?;
    update_ddns_record(domain, provider, addr).await?;
    Ok(addr)
}

async fn find_addr(config: &AddrConfig, family: Family) -> anyhow::Result<Option<IpAddr>> {
    let addr = match &config.method {
        Method::Api { endpoint } => {
            let data = reqwest::Client::new()
                .get(&*endpoint)
                .header("User-Agent", "curl/0.0.0")
                .send()
                .await?
                .bytes()
                .await?;
            let ip = String::from_utf8(data.to_vec())?;
            let addr = IpAddr::from_str(&ip)?;
            vec![addr]
        }
        Method::Nic { interface } => local_ip_address::list_afinet_netifas()?
            .into_iter()
            .filter_map(|(name, addr)| (&name == &*interface).then_some(addr))
            .collect::<Vec<IpAddr>>(),
        Method::Cmd { command } => {
            let output = Command::new("sh").args(["-c", &*command]).output().await?;
            let addr = String::from_utf8(output.stdout.to_vec())?;
            let addr = IpAddr::from_str(&addr.trim())?;
            vec![addr]
        }
    };
    Ok(filter(addr, family))
}

// #[inline]
fn filter(addrs: Vec<IpAddr>, family: Family) -> Option<IpAddr> {
    let check = |addr: &IpAddr| match family {
        Ipv4 => addr.is_ipv4(),
        Ipv6 => addr.is_ipv6(),
    };
    addrs.into_iter().filter(check).next()
}
