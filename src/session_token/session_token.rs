use chrono::{TimeZone, Utc};
use libaes::Cipher;
use my_http_server::{RequestClaim, RequestCredentials};
use sha2::{Digest, Sha512};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionToken {
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
}

impl RequestCredentials for SessionToken {
    fn get_id(&self) -> &str {
        &self.trader_id
    }

    fn get_claims(&self) -> Option<Vec<RequestClaim>> {
        None
    }
}

impl SessionToken {
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

    pub fn new_from_string(token_as_str: &str, key: &str) -> Option<SessionToken> {
        let decoded_token = base64::decode(token_as_str);

        if decoded_token.is_err() {
            return None;
        }

        let decoded_token = decoded_token.unwrap();
        let mut iv: [u8; 16] = [0; 16];
        iv.copy_from_slice(&decoded_token[..16]);

        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let cipher = Cipher::new_192(&aes_key);
        let decrypted = cipher.cbc_decrypt(&iv, &decoded_token[16..]);

        let result: Result<SessionToken, prost::DecodeError> =
            prost::Message::decode(&decrypted[..]);

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::session_token::SessionToken;

    #[test]
    fn test_decrypt() {
        let my_key = "e537d941-f7d2-4939-b97b-ae4722ca56aa";
        let token_as_str = 
"/3Lqdf/Xjmi4+z+OyUuGY5U9NEnd5BjlaWUJqisHjw3/NzlboJCgqZ0FWB/9+goxLh0hkWb2i7HTMXkUPFT3Sr+6vLjYQMZn6+OPrFmrw3o1h7UpnMwjL/JwyqhfFZjqbUN/ceDXzJJzJhSDQcEMjqA9pDQLWpVjKixWhL5jKT1/0EfQeQpaN/INn9b7CKIn4BLkcGIB/uPVKqUT0Fkdlg==";
        let token = SessionToken::new_from_string(token_as_str, my_key).unwrap();
        println!("{:#?}", token);

        assert_eq!("73dd0bf974ce47ed89606a3788917a18", token.trader_id);
        assert_eq!("9a558d487ea740b5a53ff938a139fa2e", token.brand_id);
        assert_eq!("661359502d3e4eaaaae533d5556ab164", token.ip);
        assert!(1669211025404 == token.expires_ts);
    }
}
