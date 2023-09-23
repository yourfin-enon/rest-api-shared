use isolang::Language;
use my_http_server::{HttpContext, HttpFailResult};

const LANG_HEADER: &str = "Accept-Language";
const DEFAULT_LANG: &str = "ENG";

pub trait GetLang {
    fn get_lang(&self) -> Result<String, HttpFailResult>;
}

impl GetLang for HttpContext {
    fn get_lang(&self) -> Result<String, HttpFailResult> {
        if let Some(header_value) = self.request.get_header(LANG_HEADER) {
            let lang = parse_lang(header_value);

            let Some(lang) = lang else {
                println!(
                    "Can't get LANG. Failed to parse value {}. Using default",
                    header_value
                );

                return Ok(DEFAULT_LANG.to_string());
            };

            return Ok(lang);
        }

        println!(
            "Can't get LANG. Header not found {}. Using default",
            LANG_HEADER
        );

        Ok(DEFAULT_LANG.to_string())
    }
}

fn parse_lang(src: &str) -> Option<String> {
    let splits: Vec<&str> = src.split(',').collect();
    let mut first_split = splits.first();

    let Some(lang) = first_split else {
        return None;
    };

    let lang = if lang.contains('-') {
        let splits: Vec<&str> = lang.split('-').collect();
        first_split = splits.first();

        let Some(lang) = first_split else {
            return None;
        };

        Language::from_639_1(lang)?
    } else if lang.len() == 3 {
        let from_three_letters = Language::from_639_3(lang);

        if let Some(from_three_letters) = from_three_letters {
            from_three_letters
        } else {
            Language::from_639_1(&lang[..2])?
        }
    } else {
        Language::from_639_1(lang)?
    };

    let lang = lang.to_639_3().to_uppercase();

    Some(lang)
}

#[cfg(test)]
mod test {
    use crate::http_context::get_lang::parse_lang;

    #[test]
    fn parse_uk_from_long_value() {
        let lang = parse_lang("uk-UA,uk;q=0.9,en-US;q=0.8,en;q=0.7");

        assert_eq!("UKR", lang.unwrap());
    }

    #[test]
    fn parse_uk_from_short_value() {
        let lang = parse_lang("uk");

        assert_eq!("UKR", lang.unwrap());
    }

    #[test]
    fn parse_es_from_short_value() {
        let lang = parse_lang("es");

        assert_eq!("SPA", lang.unwrap());
    }

    #[test]
    fn parse_es_from_short_value_with_trailing() {
        let lang = parse_lang("es;");

        assert_eq!("SPA", lang.unwrap());
    }

    #[test]
    fn parse_spa_from_short_value_with_trailing() {
        let lang = parse_lang("spa");

        assert_eq!("SPA", lang.unwrap());
    }
}
