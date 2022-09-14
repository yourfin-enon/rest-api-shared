pub struct TokenKey {
    pub key: String,
}

impl TokenKey {
    pub fn new(token_key: &str) -> Self {
        Self::from_string_token(token_key)
    }

    pub fn from_string_token(token_key: &str) -> Self {
        if token_key.len() != 36 {
            panic!("TOKEN_KEY is not 36 characters long");
        }
        Self {
            key: token_key.to_string(),
        }
    }
}
