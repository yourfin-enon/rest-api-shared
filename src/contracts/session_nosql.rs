service_sdk::macros::use_my_no_sql_entity!();
use serde::{Deserialize, Serialize};

#[my_no_sql_entity("client-sessions")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientSessionNosql {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "TraderId")]
    pub trader_id: String,
    #[serde(rename = "BrandId")]
    pub brand_id: String,
    #[serde(rename = "CreatedTs")]
    pub created_ts: i64,
    #[serde(rename = "DeviceUuid")]
    pub device_uuid: Option<String>,
    #[serde(rename = "UserAgent")]
    pub user_agent: Option<String>,
    #[serde(rename = "Ip")]
    pub ip: Option<String>,
    #[serde(rename = "IpCountry")]
    pub ip_country: Option<String>,
    #[serde(rename = "ExpiresTs")]
    pub expires_ts: i64,
    #[serde(rename = "LoginTwoFaConfirmed")]
    pub login_two_fa_confirmed: bool,
}

impl ClientSessionNosql {
    pub fn get_table_name() -> String {
        String::from("client-sessions")
    }

    pub fn get_partition_key(trader_id: &str) -> &str {
        trader_id
    }

    pub fn get_row_key(id: &str) -> &str {
        id
    }
}

#[my_no_sql_entity("client-sessions-lite")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteClientSessionNosql {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "TraderId")]
    pub trader_id: String,
}

impl LiteClientSessionNosql {
    pub fn get_table_name() -> String {
        String::from("client-sessions-lite")
    }

    pub fn get_partition_key(trader_id: &str) -> &str {
        trader_id
    }

    pub fn get_row_key(id: &str) -> &str {
        id
    }
}
