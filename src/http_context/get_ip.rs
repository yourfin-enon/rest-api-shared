use my_http_server::{HttpContext, HttpFailResult};

const IP_HEADERS: [&str; 4] = [
    "CF-Connecting-IP",
    "X-Forwarded-For",
    "HTTP_X_FORWARDED_FOR",
    "REMOTE_ADDR",
];

pub trait GetIp {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult>;
}

impl GetIp for HttpContext {
    fn get_country_alpha2(&self) -> Result<String, HttpFailResult> {
        for header in IP_HEADERS {
            if let Some(header_value) = self.request.get_header(header) {
                if let Ok(parsed_header_value) = std::str::from_utf8(header_value.as_bytes()) {
                    return Ok(parsed_header_value.to_owned());
                }
            }
        }

        println!("Can't get country",);

        return Ok("".to_string());
    }
}
