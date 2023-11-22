use super::ApiResultStatus;
use serde::{Deserialize, Serialize};
use service_sdk::my_http_server;
use service_sdk::my_http_server::macros::MyHttpObjectStructure;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct FailedApiResponse {
    #[serde(rename = "result")]
    pub response_code: ApiResultStatus,
    #[serde(rename = "message")]
    pub message: String,
}
