use isolang::Language;
use my_http_server::{HttpContext, HttpFailResult};

const LANG_HEADER: &str = "Accept-Language";
const DEFAULT_LANG: &str = "ENG";

pub trait GetLang {
    fn get_lang(&self) -> Result<String, HttpFailResult>;
}

impl GetLang for HttpContext {
    fn get_lang(&self) -> Result<String, HttpFailResult> {
        if let Some(header_value) = self.request.get_key_value(LANG_HEADER) {
            if let Ok(parsed_header_value) = std::str::from_utf8(header_value) {
                let split: Vec<&str> = parsed_header_value.split(",").collect();

                if let Some(main_lang) = split.first() {
                    if let Some(main_lang) = Language::from_639_1(main_lang) {
                        let main_lang = main_lang.to_639_3().to_uppercase();
                        
                        return Ok(main_lang);
                    }
                }
            }
        }

        println!("Can't get LANG. Header not found {}. Using default", LANG_HEADER);

        return Ok(DEFAULT_LANG.to_string());
    }
}
