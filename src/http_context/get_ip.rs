use service_sdk::my_http_server::{HttpContext, HttpFailResult};
service_sdk::macros::use_my_http_server!();

const IP_HEADERS: [&str; 4] = [
    "CF-Connecting-IP",
    "X-Forwarded-For",
    "HTTP_X_FORWARDED_FOR",
    "REMOTE_ADDR",
];

pub trait GetIp {
    fn get_ip(&self) -> Result<String, HttpFailResult>;
}

impl GetIp for HttpContext {
    fn get_ip(&self) -> Result<String, HttpFailResult> {
        for header in IP_HEADERS {
            let headers = self.request.get_headers();
            let header = headers.try_get_case_insensitive(header);

            if let Some(header_value) = header {
                if let Ok(parsed_header_value) = std::str::from_utf8(header_value.value) {
                    return Ok(parsed_header_value.to_owned());
                }
            }
        }

        println!("Can't get IP. None of the headers found {:?}", IP_HEADERS);

        return Ok("".to_string());
    }
}
