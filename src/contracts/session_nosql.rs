use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientSessionNosql {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
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
}

impl MyNoSqlEntity for ClientSessionNosql {
    const TABLE_NAME: &'static str = "client-sessions";

    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn get_row_key(&self) -> &str {
        &self.row_key
    }

    fn get_time_stamp(&self) -> i64 {
        self.created_ts
    }

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteClientSessionNosql {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "TraderId")]
    pub trader_id: String,
}

impl MyNoSqlEntity for LiteClientSessionNosql {
    const TABLE_NAME: &'static str = "client-sessions-lite";

    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn get_row_key(&self) -> &str {
        &self.row_key
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::now().unix_microseconds
    }
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
