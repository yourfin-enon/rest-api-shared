use chrono::{TimeZone, Utc};
use my_http_server::{RequestClaim, RequestCredentials};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use super::TokenCipher;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessToken {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub trader_id: ::prost::alloc::string::String,

    #[prost(int64, tag = "3")]
    pub expires_ts: i64,

    #[prost(string, tag = "4")]
    pub session_id: ::prost::alloc::string::String,

    #[prost(string, tag = "5")]
    pub brand_id: ::prost::alloc::string::String,

    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,

    #[prost(message, repeated, tag = "7")]
    pub claims: ::prost::alloc::vec::Vec<AccessClaim>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessClaim {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub expires: i64,
    //#[prost(message, repeated, tag = "3")]
    //pub allowed_ips: ::prost::alloc::vec::Vec<String>,
}

impl RequestCredentials for AccessToken {
    fn get_id(&self) -> &str {
        &self.trader_id
    }

    fn get_claims(&self) -> Option<Vec<RequestClaim>> {
        if self.claims.len() == 0 {
            return None;
        }
        else {
            let mapped: Vec<RequestClaim> = self.claims
            .iter()
            .map(|c| {
                let expires = DateTimeAsMicroseconds::new(Utc.timestamp_millis_opt(c.expires).single().unwrap_or_default().timestamp_micros());

                RequestClaim {
                    allowed_ips: None,
                    expires,
                    id: &c.id
                }})
                .collect();

            return Some(mapped);
        }
    }
}

impl AccessToken {
    pub fn get_brand_id(&self) -> &str {
        &self.brand_id
    }

    pub fn get_user_id(&self) -> &str {
        &self.trader_id
    }

    pub fn receive_user_id(self) -> String {
        self.trader_id
    }

    pub fn get_expires_microseconds(&self) -> i64 {
        let expires = Utc.timestamp_millis_opt(self.expires_ts).single();

        if let Some(expires) = expires {
            return expires.timestamp_micros();
        } else {
            return 0;
        }
    }

    pub fn new_from_string(token_as_str: &str, key: &str) -> Option<AccessToken> {
        let result = TokenCipher::decrypt(token_as_str, key);

        match result {
            Err(_err) => return None,
            Ok(data) => return Some(data),
        }
    }

    pub fn to_string(&self, key: &str) -> String {
        TokenCipher::encrypt(self, key)
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use my_http_server::RequestCredentials;

    use crate::token::{AccessToken, AccessClaim};

    #[test]
    fn test_encrypt_decrypt() {
        let my_key = "e537d941-f7d2-4939-b97b-ae4722ca56aa";
        let src_token_str = 
"2KaGFpk+Maqg6Qdh2Axd9o5xyA6obs0gvKDteB/IHzhFk5rQWAAztfsPoqdausKyblkZLOecQphjm83gxJBZ0oyrY82yRsdTpUBZfagozqbM4RMmMfFoMw4Kc6BrDajeXEIJFhyVpq1qiO6MbauKJnOPtM/mNvIsTZ7WDgSpDLx2dkDheWkbKxOAEhOwa5GxdAlS+cQQyiEXSVEngnEciKTnl5w/9gx5b8UC+IBb3P9obSTOhj6uqRbkHuQ7fmdm";
        let src_token = AccessToken::new_from_string(src_token_str, my_key).unwrap();
        let encrypted_token_str = src_token.to_string(my_key);
        let decrypted_token = AccessToken::new_from_string(&encrypted_token_str, my_key).unwrap();

        assert_eq!(src_token.trader_id, decrypted_token.trader_id);
        assert_eq!(src_token.brand_id, decrypted_token.brand_id);
    }

    #[test]
    fn test_decrypt() {
        let my_key = "e537d941-f7d2-4939-b97b-ae4722ca56aa";
        let token_as_str = 
"2KaGFpk+Maqg6Qdh2Axd9o5xyA6obs0gvKDteB/IHzhFk5rQWAAztfsPoqdausKyblkZLOecQphjm83gxJBZ0oyrY82yRsdTpUBZfagozqbM4RMmMfFoMw4Kc6BrDajeXEIJFhyVpq1qiO6MbauKJnOPtM/mNvIsTZ7WDgSpDLx2dkDheWkbKxOAEhOwa5GxdAlS+cQQyiEXSVEngnEciKTnl5w/9gx5b8UC+IBb3P9obSTOhj6uqRbkHuQ7fmdm";
        let token = AccessToken::new_from_string(token_as_str, my_key).unwrap();
        println!("{:#?}", token);
        let creds: Box::<dyn RequestCredentials> = Box::new(token.clone());
        let creds_claims = creds.get_claims().unwrap();
        let claim_ids: Vec<String> = creds_claims.iter().map(|v| {v.id.to_string()}).collect();
        println!("{:#?}", claim_ids);

        assert_eq!("c529bf7411fd46619b1b08ce6c17633d", token.trader_id);
        assert_eq!("Monfex", token.brand_id);
        assert_eq!("143.244.46.213", token.ip);
        assert_eq!(1, token.claims.len());
        assert_eq!("EmailConfirmed", token.claims.first().unwrap().id);
        assert_eq!(1670438768896, token.expires_ts);
    }

    #[test]
    fn test_get_claims() {
        let token = AccessToken {
            claims: vec![AccessClaim {
                //allowed_ips: vec!["1".to_string(), "2".to_string()],
                expires: Utc::now().timestamp_millis(),
                id: "Test".to_string()
            }],
            ..Default::default()
        };
        let creds: Box::<dyn RequestCredentials> = Box::new(token.clone());
        let creds_claims = creds.get_claims().unwrap();

        assert_eq!(creds_claims.len(), token.claims.len());
        assert_eq!(creds_claims.get(0).unwrap().id, token.claims.get(0).unwrap().id);
        assert_eq!(creds_claims.get(0).unwrap().expires.to_chrono_utc().timestamp_millis(), token.claims.get(0).unwrap().expires);
    }
}
