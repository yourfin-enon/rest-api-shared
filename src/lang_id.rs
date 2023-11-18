use service_sdk::my_http_server::HttpContext;

const ENGLISH: &str = "en";

pub enum LanguageId {
    English,
}

impl LanguageId {
    pub fn as_str(&self) -> &str {
        match self {
            LanguageId::English => ENGLISH,
        }
    }
}

impl Default for LanguageId {
    fn default() -> Self {
        Self::English
    }
}

pub trait GetLanguageId {
    fn get_language_id(&self) -> LanguageId;
}

impl GetLanguageId for HttpContext {
    fn get_language_id(&self) -> LanguageId {
        LanguageId::default()
    }
}
