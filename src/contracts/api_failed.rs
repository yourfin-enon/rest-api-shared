use my_http_server_swagger::MyHttpObjectStructure;
use serde::{Deserialize, Serialize};
use super::ApiResultStatus;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct FailedApiResponse {
    #[serde(rename = "result")]
    pub response_code: ApiResultStatus,
    #[serde(rename = "message")]
    pub message: String,
}