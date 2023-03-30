use super::TokenCipher;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshToken {
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(string, tag = "2")]
    pub trader_id: String,

    #[prost(string, tag = "3")]
    pub session_id: String,

    #[prost(string, tag = "4")]
    pub ip: String,
    
    #[prost(int64, tag = "5")]
    pub expires_ts: i64,

}

impl RefreshToken {
    pub fn get_user_id(&self) -> &str {
        &self.trader_id
    }

    pub fn receive_user_id(self) -> String {
        self.trader_id
    }

    pub fn get_expires_microseconds(&self) -> i64 {
        self.expires_ts * 1000
    }

    pub fn new_from_string(token_as_str: &str, key: &str) -> Option<RefreshToken> {
        let result = TokenCipher::decrypt(token_as_str, key);

        return match result {
            Err(_err) => None,
            Ok(data) => Some(data),
        }
    }

    pub fn to_string(&self, key: &str) -> String {
        TokenCipher::encrypt(self, key)
    }
}

#[cfg(test)]
mod test {
    use crate::token::refresh_token::RefreshToken;

    #[test]
    fn test_encrypt_decrypt() {
        let my_key = "e537d941-f7d2-4939-b97b-ae4722ca56aa";
        let src_token_str =
        "AeVCrDOwv5X/rSazlr+pdjyrPg5dDYdOIuhY40HVSXcGKWJz1mvb+n+MnXdSjj499V5bGDKv1BsqbZxKfakR+glJ8meB4ZSC4eVcap/NM4PRURWP4VfbufRu7QbLFKS8nmwz0FP585GB89a5Ivrztn0+nkFYNDmalsyV0rS/T506OBkO51QzIw1gi+5Q8t8OB+M3sn7JTcNGFtB0BIJycA==";
        let src_token = RefreshToken::new_from_string(src_token_str, my_key).unwrap();
        let encrypted_token_str = src_token.to_string(my_key);
        let decrypted_token = RefreshToken::new_from_string(&encrypted_token_str, my_key).unwrap();

        assert_eq!(src_token.trader_id, decrypted_token.trader_id);
    }

    #[test]
    fn test_decrypt() {
        let my_key = "e537d941-f7d2-4939-b97b-ae4722ca56aa";
        let token_as_str =
            "AeVCrDOwv5X/rSazlr+pdjyrPg5dDYdOIuhY40HVSXcGKWJz1mvb+n+MnXdSjj499V5bGDKv1BsqbZxKfakR+glJ8meB4ZSC4eVcap/NM4PRURWP4VfbufRu7QbLFKS8nmwz0FP585GB89a5Ivrztn0+nkFYNDmalsyV0rS/T506OBkO51QzIw1gi+5Q8t8OB+M3sn7JTcNGFtB0BIJycA==";
        let token = RefreshToken::new_from_string(token_as_str, my_key).unwrap();
        println!("{:#?}", token);

        assert_eq!("e04156d6d2d648679da508b6c2e27a54", token.trader_id);
        assert_eq!("176.52.76.7", token.ip);
        assert_eq!(1700904396080, token.expires_ts);
    }
}
