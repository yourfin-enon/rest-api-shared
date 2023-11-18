use service_sdk::my_http_server::{HttpContext, HttpFailResult};

const HEADER: &str = "User-Agent";

pub trait GetUserAgent {
    fn get_user_agent(&self) -> Result<String, HttpFailResult>;
}

impl GetUserAgent for HttpContext {
    fn get_user_agent(&self) -> Result<String, HttpFailResult> {
        if let Some(header_value) = self.request.get_header(HEADER) {
            if let Ok(parsed_header_value) = std::str::from_utf8(header_value.as_bytes()) {
                return Ok(parsed_header_value.to_owned());
            }
        }

        println!("Can't get user agent. Header not found: {}", HEADER);

        return Ok("".to_string());
    }
}
