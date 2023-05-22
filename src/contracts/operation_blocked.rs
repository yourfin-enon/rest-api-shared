use my_http_server_swagger::MyHttpObjectStructure;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};
use super::ApiResultStatus;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct OperationBlockedApiResponse {
    #[serde(rename = "result")]
    pub result: ApiResultStatus,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "data")]
    pub data: OperationBlockedApiData,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct OperationBlockedApiData {
    #[serde(rename = "expireDate")]
    pub expire_date: i64,
}

impl OperationBlockedApiResponse {
    pub fn new(expire_date: DateTimeAsMicroseconds) -> Self {
        Self {
            result: ApiResultStatus::OperationBlocked,
            description: "Operation blocked".to_string(),
            data: OperationBlockedApiData { expire_date: expire_date.unix_microseconds / 1000 },
        }
    }
}