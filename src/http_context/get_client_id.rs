use service_sdk::my_http_server::{HttpContext, HttpFailResult};
use crate::middlewares::KV_SESSION_ID;

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

pub trait GetSessionId {
    fn get_session_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetSessionId for HttpContext {
    fn get_session_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(client_id) = self.request.get_key_value(KV_SESSION_ID) {
            let result = std::str::from_utf8(client_id).unwrap();
            return Ok(result);
        }

        return Err(HttpFailResult::as_unauthorized(
            Some("Can't get session id looks like request is unauthorised"
                .to_string()),
        ));    }
}
