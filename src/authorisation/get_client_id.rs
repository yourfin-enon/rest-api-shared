use my_http_server::{HttpContext, HttpFailResult};

use crate::middlewares::KV_USER_ID;

pub trait GetClientId {
    fn get_client_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetClientId for HttpContext {
    fn get_client_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(client_id) = self.request.get_key_value(KV_USER_ID) {
            let result = std::str::from_utf8(client_id).unwrap();
            return Ok(result);
        }

        return Err(HttpFailResult::as_unauthorized(
            "Can not get client Id. Looks like request is unathorised"
                .to_string()
                .into(),
        ));
    }
}
