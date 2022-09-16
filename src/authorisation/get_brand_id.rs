use my_http_server::{HttpContext, HttpFailResult};

use crate::middlewares::KV_BRAND_ID;

pub trait GetBrandId {
    fn get_brand_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetBrandId for HttpContext {
    fn get_brand_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(client_id) = self.request.get_key_value(KV_BRAND_ID) {
            let result = std::str::from_utf8(client_id).unwrap();
            return Ok(result);
        }

        return Err(HttpFailResult::as_unauthorized(
            "Can not get brand id Looks like request is unathorised"
                .to_string()
                .into(),
        ));
    }
}
