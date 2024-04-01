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
