use super::HOST;
use crate::provider::{
    digest::{hex_sha256, hmac_sha256},
    tencent::Config,
};
use indoc::formatdoc;
use time::UtcDateTime;

/// https://cloud.tencent.com/document/product/213/30654
pub(super) fn create_signature(
    config: &Config,
    timestamp: i64,
    action: &str,
    payload: &str,
) -> anyhow::Result<String> {
    let time = UtcDateTime::from_unix_timestamp(timestamp)?;
    let timestamp = time.unix_timestamp().to_string();
    let date = time.date().to_string();

    let signed_header_names = "content-type;host;x-tc-action";
    let signed_header = formatdoc!(
        "content-type:application/json; charset=utf-8
         host:{}
         x-tc-action:{}
         ",
        HOST,
        action.to_lowercase()
    );
    let req = [
        "POST",
        "/",
        "",
        &signed_header,
        &signed_header_names,
        &hex_sha256(&payload[..]),
    ]
    .join("\n");

    let credit_scope = format!("{date}/dnspod/tc3_request");
    let req = [
        "TC3-HMAC-SHA256",
        &timestamp,
        &credit_scope,
        &hex_sha256(&req[..]),
    ]
    .join("\n");

    let key = format!("TC3{}", config.secret_key);
    let sig_date: [u8; 32] = hmac_sha256(key, date)?;
    let sig_svc: [u8; 32] = hmac_sha256(sig_date, "dnspod")?;
    let key: [u8; 32] = hmac_sha256(sig_svc, "tc3_request")?;
    let sig = hex::encode(&hmac_sha256(&key, req)?[..]);

    let authorization = format!(
        "TC3-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
        &config.secret_id, &credit_scope, &signed_header_names, &sig
    );
    Ok(authorization)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::tencent::{Config, client::schema::DescribeRecordList};

    #[test]
    fn test_signature() {
        let config = Config {
            secret_id: "AKID********************************".to_string(),
            secret_key: "********************************".to_string(),
        };
        let time = 1551113065;
        let data = serde_json::to_string(&DescribeRecordList {
            domain: "zhouxi.me",
            record_type: "A",
            subdomain: "@",
        })
        .unwrap();
        let sig = "TC3-HMAC-SHA256 Credential=AKID********************************/2019-02-25/dnspod/tc3_request, SignedHeaders=content-type;host;x-tc-action, Signature=dac9cc8e9da678e46365285043b2e2f236868c85385c3919ea9f98df14863fd9";
        let signature = create_signature(&config, time, "DescribeInstances", &data).unwrap();
        assert_eq!(signature, sig);
    }
}
