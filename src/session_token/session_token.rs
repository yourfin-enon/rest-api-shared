use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::TokenKey;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionToken {
    #[prost(string, tag = "1")]
    user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    expires: i64,
    #[prost(string, tag = "3")]
    ip: ::prost::alloc::string::String,
}

impl SessionToken {
    pub fn new(user_id: String, expires: DateTimeAsMicroseconds, ip: String) -> Self {
        SessionToken {
            user_id,
            expires: expires.unix_microseconds,
            ip,
        }
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn receive_user_id(self) -> String {
        self.user_id
    }

    pub fn get_expires_microseconds(&self) -> i64 {
        self.expires
    }

    pub fn into_token(&self, token_key: &TokenKey) -> String {
        let mut token_payload = Vec::new();
        prost::Message::encode(self, &mut token_payload).unwrap();

        let ciphertext = enc_file::encrypt_aes(token_payload, token_key.key.as_str()).unwrap();

        base64::encode(ciphertext)
    }

    pub fn parse_from_token(token_as_str: &str, token_key: &TokenKey) -> Option<SessionToken> {
        let encoded_token = base64::decode(token_as_str);

        if encoded_token.is_err() {
            return None;
        }

        let result = enc_file::decrypt_aes(encoded_token.unwrap(), token_key.key.as_str());

        if result.is_err() {
            return None;
        }

        let result: Result<SessionToken, prost::DecodeError> =
            prost::Message::decode(result.unwrap().as_slice());

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_encrypt_decrypt() {
        use super::*;
        use crate::session_token::TokenKey;

        let token_key = TokenKey::from_string_token("an exampaaaaaaaaaaaaaaaaaaaaaaaa");

        let session_token = SessionToken::new(
            "user_id".to_string(),
            DateTimeAsMicroseconds::now(),
            "127.0.0.1".to_string(),
        );

        let token_as_str = session_token.into_token(&token_key);

        let session_token_from_token = SessionToken::parse_from_token(&token_as_str, &token_key);

        print!("{:?}", session_token_from_token);

        assert_eq!(session_token, session_token_from_token.unwrap());
    }
}
