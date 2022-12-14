use my_http_server::{HttpContext, HttpFailResult};

const HEADER: &str = "cf-ipcountry";

pub trait GetCountry {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult>;
}

impl GetCountry for HttpContext {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult> {
        if let Some(header_value) = self.request.get_headers().get(HEADER) {
            if let Ok(parsed_header_value) = std::str::from_utf8(header_value.as_bytes()) {
                return Ok(parsed_header_value.to_owned());
            }
        }

        return Ok("".to_string());
    }
}