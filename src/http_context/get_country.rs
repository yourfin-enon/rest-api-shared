use service_sdk::my_http_server::{HttpContext, HttpFailResult};

const COUNTRY_HEADERS: [&str; 2] = [
    "cf-ipcountry",
    "http_cf_ipcountry",
];

pub trait GetCountry {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult>;
    fn get_country_alpha3(&self) -> Result<String, HttpFailResult>;
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

        println!("Can't get_country_alpha2. Non of the header found:  {:?}", COUNTRY_HEADERS);

        return Ok("".to_string());
    }

    fn get_country_alpha3(&self) -> Result<String, HttpFailResult> {
        let alpha2_code = self.get_country_alpha2();

        let Ok(alpha2_code) = alpha2_code else {
            return alpha2_code;
        };

        let country = rust_iso3166::from_alpha2(&alpha2_code);

        let Some(country) = country else {
            println!("Can't get_country_alpha3. Not found alpha 3 code for {:?}", alpha2_code);

            return Ok("".to_string());
        };

        return Ok(country.alpha3.to_string());
    }
}