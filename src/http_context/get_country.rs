use my_http_server::{HttpContext, HttpFailResult};

const HEADER_1: &str = "cf-ipcountry";
const HEADER_2: &str = "http_cf_ipcountry";

pub trait GetCountry {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult>;
}

impl GetCountry for HttpContext {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult> {
        if let Some(header_value) = self.request.get_header(HEADER_1) {
            if let Ok(parsed_header_value) = std::str::from_utf8(header_value.as_bytes()) {
                return Ok(parsed_header_value.to_owned());
            }
        }

        if let Some(header_value) = self.request.get_header(HEADER_2) {
            if let Ok(parsed_header_value) = std::str::from_utf8(header_value.as_bytes()) {
                return Ok(parsed_header_value.to_owned());
            }
        }
        
        println!("Can't get country. Not found header {} nor {}", HEADER_1, HEADER_2);

        return Ok("".to_string());
    }
}