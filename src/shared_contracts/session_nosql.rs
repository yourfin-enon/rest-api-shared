use my_no_sql_server_abstractions::MyNoSqlEntity;
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
    pub device_uuid: String,
    #[serde(rename = "UserAgent")]
    pub user_agent: String,
    #[serde(rename = "Ip")]
    pub ip: String,
    #[serde(rename = "IpCountry")]
    pub ip_country: String,
}

impl MyNoSqlEntity for ClientSessionNosql {
    fn get_partition_key(&self) -> &str {
        &self.trader_id
    }

    fn get_row_key(&self) -> &str {
        "*"
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
