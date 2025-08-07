use serde::Deserialize;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(Deserialize, Debug)]
pub struct AliyunError {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}

impl Display for AliyunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("AliyunError: Code: ")?;
        f.write_str(&self.code)?;
        f.write_str(". Message: ")?;
        f.write_str(&self.message)?;
        Ok(())
    }
}

impl Error for AliyunError {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DnsRecord {
    pub value: String,
    pub record_id: String,
    pub rr: String,
}

#[derive(Deserialize)]
pub struct RecordId {
    #[serde(rename = "RecordId")]
    pub id: String,
}
impl Display for RecordId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}
