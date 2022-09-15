use libaes::Cipher;
use sha2::{Digest, Sha512};
use std::{
    time::{Duration},
};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionToken {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,

    #[prost(message, tag = "2")]
    pub expires: ::core::option::Option<prost_types::Timestamp>,

    #[prost(string, tag = "13")]
    pub brand_id: ::prost::alloc::string::String,

    #[prost(string, tag = "14")]
    pub ip: ::prost::alloc::string::String,
}

impl SessionToken {
    pub fn get_user_id(&self) -> &str {
        &self.id
    }

    pub fn receive_user_id(self) -> String {
        self.id
    }

    pub fn get_expires_microseconds(&self) -> i64 {
        let expires_ts = self.expires.as_ref().unwrap();
        let duration = Duration::new(expires_ts.seconds as u64, expires_ts.nanos as u32);

        duration.as_micros() as i64
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
        let decrypted = cipher.cbc_decrypt(&iv, &decoded_token[16..128]);

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
    use std::time::Duration;

    use crate::session_token::SessionToken;

    #[test]
    fn test_decrypt() {
        let my_key = "e537d941-f7d2-4939-b97b-ae4722ca56aa";
        let token_as_str = "8hXbbNNgNVQU+p5NhLIM/83EgxK2yn2WAYppHFNY1B3w7tXDULg8XBv/WDD309QKyydPjbn8dXkrl8sLVrHHYTVqQA7B3FwtluqqdiqKxUifO9sOFdIwRnQ3/tgXYnbUZxpJyT5lsHsBTdIWaQ5WYVACJGaoZVO5uMvOZyQl2fU=";

        let token = SessionToken::new_from_string(token_as_str, my_key).unwrap();

        assert_eq!("9674f28758644015930dd836e43bacef", token.get_user_id());
        assert_eq!("Monfex", token.brand_id);
        assert_eq!("176.52.29.155", token.ip);
        assert!(1663270043578898 < token.get_expires_microseconds());
    }
}
