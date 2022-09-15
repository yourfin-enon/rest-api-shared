use libaes::Cipher;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use sha2::{Digest, Sha512};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionToken {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,

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
        DateTimeAsMicroseconds::now().unix_microseconds + 1000 * 60 *60
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

        let result: Result<SessionToken, prost::DecodeError> = prost::Message::decode(&decrypted[..]);

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
        let token_as_str = "mFKXtvMOmCaV/JTeGpyq+C6AB83s6/HtsQfowZGMX+M0TfgXrzspR3exyYNGRkILe9T5rpgZLlWUWUF4vFc/XxIPGWEd7KPaiXlUcpAWOJ8TVrP4z0KkgOjjuH1TUBopQJ0LtQeiG906ZIcvkTrNP198ypq+EAvA8jVoKWaeonc=";
        
        let token = SessionToken::new_from_string(token_as_str, my_key).unwrap();

        assert_eq!("9674f28758644015930dd836e43bacef", token.get_user_id());
        assert_eq!("Monfex", token.brand_id);
        assert_eq!("176.52.29.155", token.ip);
    }
}
