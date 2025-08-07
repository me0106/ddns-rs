use crate::provider::{
    aliyun::{
        Config,
        client::{HOST, VERSION},
    },
    digest::{hex_sha256, hmac_sha256},
};
use indoc::formatdoc;

/// https://help.aliyun.com/zh/sdk/product-overview/v3-request-structure-and-signature
pub(super) fn create_signature(
    config: &Config,
    timestamp: &str,
    action: &str,
    query: &str,
    hashed_body: &str,
    random: &str,
) -> anyhow::Result<String> {
    let signed_header_names =
        "host;x-acs-action;x-acs-content-sha256;x-acs-date;x-acs-signature-nonce;x-acs-version";
    let signed_headers = formatdoc!(
        "host:{}
         x-acs-action:{}
         x-acs-content-sha256:{}
         x-acs-date:{}
         x-acs-signature-nonce:{}
         x-acs-version:{}
         ",
        HOST,
        action,
        hashed_body,
        timestamp,
        random,
        VERSION
    );

    let canonical_request = [
        "POST",
        "/",
        query,
        &signed_headers,
        &signed_header_names,
        &hashed_body,
    ]
    .join("\n");

    let req = format!("ACS3-HMAC-SHA256\n{}", &hex_sha256(canonical_request));
    let signed = hex::encode(hmac_sha256(&config.secret_key, &req)?);

    let authorization = format!(
        "ACS3-HMAC-SHA256 Credential={},SignedHeaders={},Signature={}",
        &config.secret_id, &signed_header_names, &signed
    );
    Ok(authorization)
}

#[cfg(test)]
mod tests {
    use super::{super::Config, create_signature};
    use time::{UtcDateTime, format_description::well_known::Rfc3339};

    #[test]
    fn test() {
        let date = UtcDateTime::from_unix_timestamp(1698315752).unwrap();
        let signature = create_signature(
            &Config {
                secret_id: "YourAccessKeyId".to_string(),
                secret_key: "YourAccessKeySecret".to_string(),
            },
            &date.format(&Rfc3339).unwrap(),
            "RunInstances",
            "",
            "",
            "3156853299f313e23d1673dc12e1703d",
        )
        .unwrap();
        assert_eq!(
            signature,
            "ACS3-HMAC-SHA256 Credential=YourAccessKeyId,SignedHeaders=host;x-acs-action;x-acs-content-sha256;x-acs-date;x-acs-signature-nonce;x-acs-version,Signature=a7537a2e570f3c1edb748445311eee3e3bf1e6cd30ea2dd8bdb5c8e45430b112"
        )
    }
}
