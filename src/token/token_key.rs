pub struct TokenKey {
    pub key: String,
}

impl TokenKey {
    pub fn new(token_key: &str) -> Self {
        Self::from_string_token(token_key)
    }

    pub fn from_string_token(token_key: &str) -> Self {
        
        Self {
            key: token_key.to_string(),
        }
    }
}
