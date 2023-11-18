use serde::{Deserialize, Serialize};
use service_sdk::my_http_server::macros::MyHttpObjectStructure;
use super::ApiResultStatus;
use service_sdk::my_http_server;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct FailedApiResponse {
    #[serde(rename = "result")]
    pub response_code: ApiResultStatus,
    #[serde(rename = "message")]
    pub message: String,
}