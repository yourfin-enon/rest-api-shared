use my_http_server::{HttpContext, HttpFailResult};

pub trait GetClientId {
    fn get_client_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetClientId for HttpContext {
    fn get_client_id(&self) -> Result<&str, HttpFailResult> {
        match &self.credentials {
            Some(value) => {
                return Ok(value.get_id());
            }
            None => Err(HttpFailResult::as_unauthorized(
                Some("User id is not found".to_string()),
            )),
        }
    }
}
