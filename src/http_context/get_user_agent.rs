use service_sdk::my_http_server::{HttpContext, HttpFailResult};
service_sdk::macros::use_my_http_server!();

const HEADER: &str = "User-Agent";

pub trait GetUserAgent {
    fn get_user_agent(&self) -> Result<String, HttpFailResult>;
}

impl GetUserAgent for HttpContext {
    fn get_user_agent(&self) -> Result<String, HttpFailResult> {
        let headers = self.request.get_headers();
        let header = headers.try_get_case_insensitive(HEADER);

        if let Some(header_value) = header {
            if let Ok(parsed_header_value) = std::str::from_utf8(header_value.value) {
                return Ok(parsed_header_value.to_owned());
            }
        }

        println!("Can't get user agent. Header not found: {}", HEADER);

        return Ok("".to_string());
    }
}
