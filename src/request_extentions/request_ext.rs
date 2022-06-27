use my_http_server::{HttpFailResult, HttpRequest};

pub trait RequestExtentions {
    fn get_user_id(&self) -> Result<&str, HttpFailResult>;
}

impl RequestExtentions for HttpRequest {
    fn get_user_id(&self) -> Result<&str, HttpFailResult> {
        match self.get_key_value(crate::middlewares::KV_USER_ID) {
            Some(value) => Ok(std::str::from_utf8(value).unwrap()),
            None => Err(HttpFailResult::as_unauthorized(
                "User id is not found".to_string().into(),
            )),
        }
    }
}
