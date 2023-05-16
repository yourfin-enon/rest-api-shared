use my_http_server::{HttpContext, HttpFailResult};

const COUNTRY_HEADERS: [&str; 2] = [
    "cf-ipcountry",
    "http_cf_ipcountry",
];

pub trait GetCountry {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult>;
}

impl GetCountry for HttpContext {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult> {
        for header in COUNTRY_HEADERS {
            if let Some(header_value) = self.request.get_header(header) {
                if let Ok(parsed_header_value) = std::str::from_utf8(header_value.as_bytes()) {
                    return Ok(parsed_header_value.to_owned());
                }
            }
        }

        println!("Can't get country. Non of the header found:  {:?}", COUNTRY_HEADERS);

        return Ok("".to_string());
    }
}