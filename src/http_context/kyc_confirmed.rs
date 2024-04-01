use crate::middlewares::{KV_BRAND_ID, KV_KYC_ID, KV_SESSION_ID};
use service_sdk::my_http_server::{HttpContext, HttpFailResult};

pub trait KycConfirmed {
    fn kyc_confirmed(&self) -> Result<bool, HttpFailResult>;
}

impl KycConfirmed for HttpContext {
    fn kyc_confirmed(&self) -> Result<bool, HttpFailResult> {
        if let Some(_kyc) = self.request.get_key_value(KV_KYC_ID) {
            return Ok(true);
        }

        return Ok(false);
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

        return Err(HttpFailResult::as_unauthorized(Some(
            "Can't get session id looks like request is unauthorised".to_string(),
        )));
    }
}
