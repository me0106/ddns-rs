use serde::{
    Deserialize, Deserializer, Serialize,
    de::{Error, IntoDeserializer},
};
use serde_json::{Map, Value};
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "Response")]
    pub response: InnerResponse<T>,
}

#[derive(Debug)]
pub struct InnerResponse<T> {
    pub content: Result<T, TencentError>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for InnerResponse<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut value: Map<String, Value> = Deserialize::deserialize(deserializer)?;
        let Some(_) = value.remove("RequestId") else {
            return Err(Error::missing_field("RequestId"));
        };

        let content = match value.remove("Error") {
            None => Ok(T::deserialize(value).map_err(|e| Error::custom(e))?),
            Some(error) => Err(TencentError {
                error: ErrorInfo::deserialize(error.into_deserializer())
                    .map_err(|e| Error::custom(e))?,
            }),
        };
        Ok(InnerResponse { content })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TencentError {
    #[serde(rename = "Error")]
    pub error: ErrorInfo,
}
impl std::error::Error for TencentError {}

impl Display for TencentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "call tencent api error. code:{},message:{}",
            self.error.code, self.error.message
        ))
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}
#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "RecordId")]
    pub id: u32,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Line")]
    pub line: String,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct RecordId {
    #[serde(rename = "RecordId")]
    pub id: u32,
}
impl Deref for RecordId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
#[derive(Serialize)]
pub struct DescribeRecordList<'a> {
    #[serde(rename = "Domain")]
    pub domain: &'a str,
    #[serde(rename = "Subdomain")]
    pub subdomain: &'a str,
    #[serde(rename = "RecordType")]
    pub record_type: &'a str,
}
#[derive(Serialize)]
pub struct ModifyRecord<'a> {
    #[serde(rename = "Domain")]
    pub domain: &'a str,
    #[serde(rename = "SubDomain")]
    pub subdomain: &'a str,
    #[serde(rename = "RecordType")]
    pub record_type: &'a str,
    #[serde(rename = "RecordLine")]
    pub record_line: &'a str,
    #[serde(rename = "Value")]
    pub value: &'a str,
    #[serde(rename = "RecordId")]
    pub record_id: u32,
}
#[derive(Serialize)]
pub struct CreateRecord<'a> {
    #[serde(rename = "Domain")]
    pub domain: &'a str,
    #[serde(rename = "RecordType")]
    pub record_type: &'a str,
    #[serde(rename = "RecordLine")]
    pub record_line: &'a str,
    #[serde(rename = "Value")]
    pub value: &'a str,
    #[serde(rename = "SubDomain")]
    pub subdomain: &'a str,
}
