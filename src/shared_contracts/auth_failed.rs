use std::{str::FromStr};
use my_http_server::HttpFailResult;
use my_http_server_swagger::MyHttpObjectStructure;
use serde::{Deserialize, Serialize};
use crate::session_token::access_claim::AccessClaimType;

use super::ApiResultStatus;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct AuthenticationFailedApiResponse {
    #[serde(rename = "result")]
    pub result: ApiResultStatus,
    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct AuthorizationFailedApiResponse {
    #[serde(rename = "result")]
    pub result: ApiResultStatus,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "data")]
    pub data: AuthorizationFailedData,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct AuthorizationFailedData {
    #[serde(rename = "requiredClaim")]
    pub required_claim: AccessClaimType,
    #[serde(rename = "description")]
    pub description: String,
}

impl AuthorizationFailedApiResponse {
    pub fn new(
        result: ApiResultStatus,
        description: String,
        data: AuthorizationFailedData,
    ) -> HttpFailResult {
        let result = AuthorizationFailedApiResponse {
            result,
            description,
            data,
        };

        let content = serde_json::to_vec(&result).unwrap();
        HttpFailResult {
            content_type: my_http_server::WebContentType::Json,
            status_code: 403,
            content,
            write_telemetry: false,
            write_to_log: false,
        }
    }

    pub fn default_desc() -> String {
        "Authorization required".to_string()
    }
}

impl AuthenticationFailedApiResponse {
    pub fn new(result: ApiResultStatus, description: String) -> HttpFailResult {
        let result = AuthenticationFailedApiResponse {
            result,
            description,
        };

        let content = serde_json::to_vec(&result).unwrap();
        HttpFailResult {
            content_type: my_http_server::WebContentType::Json,
            status_code: 401,
            content,
            write_telemetry: false,
            write_to_log: false,
        }
    }

    pub fn default_desc() -> String {
        "Authentication required".to_string()
    }
}

use my_http_server_controllers::controllers::documentation::{
    data_types::HttpDataType, out_results::HttpResult,
};

pub struct AuthFailResponseFactory;

impl my_http_server_controllers::controllers::AuthErrorFactory for AuthFailResponseFactory {
    fn get_not_authenticated(&self) -> my_http_server::HttpFailResult {
        return AuthenticationFailedApiResponse::new(
            ApiResultStatus::AccessTokenInvalid,
            AuthenticationFailedApiResponse::default_desc(),
        );
    }

    fn get_not_authorized(&self, claim_name: String) -> my_http_server::HttpFailResult {
        return AuthorizationFailedApiResponse::new(
            ApiResultStatus::AccessClaimRequired,
            AuthorizationFailedApiResponse::default_desc(),
            AuthorizationFailedData {
                required_claim: AccessClaimType::from_str(&claim_name)
                    .expect(&format!("Failed to parse claim {}", claim_name)),
                description: format!("Required access claim: {}", claim_name),
            },
        );
    }
    fn get_global_http_fail_result_types(&self) -> Option<Vec<HttpResult>> {
        let authentication_http_structure =
            AuthenticationFailedApiResponse::get_http_data_structure();
        let authorization_http_structure =
            AuthorizationFailedApiResponse::get_http_data_structure();

        Some(vec![
            HttpResult {
                http_code: 401,
                nullable: false,
                description: AuthenticationFailedApiResponse::default_desc(),
                data_type: HttpDataType::Object(authentication_http_structure),
            },
            HttpResult {
                http_code: 403,
                nullable: false,
                description: AuthorizationFailedApiResponse::default_desc(),
                data_type: HttpDataType::Object(authorization_http_structure),
            },
        ])
    }
}