use my_http_server::{HttpContext, HttpFailResult};

const HEADER: &str = "cf-ipcountry";

pub trait GetCountry {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult>;
}

impl GetCountry for HttpContext {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult> {
        let headers = self.request.get_headers();
        if let Some(header_value) = headers.get(HEADER) {
            if let Ok(parsed_header_value) = std::str::from_utf8(header_value.as_bytes()) {
                return Ok(parsed_header_value.to_owned());
            }
        }
        
        println!("Can't get country. Header cf-ipcountry not found. Available Headers: {:?}", headers);

        return Ok("".to_string());
    }
}