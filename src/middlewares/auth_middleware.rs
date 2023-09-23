use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpPath, HttpServerMiddleware,
    HttpServerRequestFlow,
};
use my_no_sql_tcp_reader::MyNoSqlDataReader;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    token::{AccessToken, TokenKey},
    contracts::{
        auth_failed::AuthenticationFailedApiResponse, ApiResultStatus, ClientSessionNosql,
        LiteClientSessionNosql,
    },
};

const AUTH_HEADER: &str = "authorization";
pub const KV_BRAND_ID: &str = "BRAND_ID";
pub const KV_SESSION_ID: &str = "SESSION_ID";

pub struct AuthMiddleware {
    token_key: TokenKey,
    ignore_full_paths: Option<Vec<HttpPath>>,
    ignore_start_path: Option<Vec<HttpPath>>,
    sessions_reader: Arc<MyNoSqlDataReader<LiteClientSessionNosql>>,
}

impl AuthMiddleware {
    pub fn new(
        token_key: TokenKey,
        sessions_reader: Arc<MyNoSqlDataReader<LiteClientSessionNosql>>,
    ) -> Self {
        Self {
            token_key,
            ignore_full_paths: None,
            ignore_start_path: None,
            sessions_reader,
        }
    }

    pub fn new_with_default_paths_to_ignore(
        token_key: TokenKey,
        sessions_reader: Arc<MyNoSqlDataReader<LiteClientSessionNosql>>,
    ) -> Self {
        let mut result = Self::new(token_key, sessions_reader);
        result.add_start_path_to_ignore("/swagger");
        result.add_start_path_to_ignore("/api/metrics");

        result
    }

    pub fn path_is_ignored(&self, http_path: &HttpPath) -> bool {
        if let Some(ref items) = self.ignore_full_paths {
            for full_path_to_ignore in items {
                if http_path.is_the_same_to(full_path_to_ignore) {
                    return true;
                }
            }
        }

        if let Some(ref items) = self.ignore_start_path {
            for start_path_to_ignore in items {
                if http_path.is_starting_with(start_path_to_ignore) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn add_full_path_to_ignore(&mut self, path: &str) {
        if self.ignore_full_paths.is_none() {
            self.ignore_full_paths = Some(Vec::new());
        }

        self.ignore_full_paths
            .as_mut()
            .unwrap()
            .push(HttpPath::from_str(path));
    }

    pub fn add_start_path_to_ignore(&mut self, path: &str) {
        if self.ignore_start_path.is_none() {
            self.ignore_start_path = Some(Vec::new());
        }

        self.ignore_start_path
            .as_mut()
            .unwrap()
            .push(HttpPath::from_str(path));
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        if self.path_is_ignored(&ctx.request.http_path) {
            return get_next.next(ctx).await;
        }

        return match ctx.request.get_header(AUTH_HEADER) {
            Some(header) => {
                if let Some(session_token) = AccessToken::new_from_string(
                    std::str::from_utf8(extract_token(header.as_bytes())).unwrap(),
                    &self.token_key.key,
                ) {
                    let pk = ClientSessionNosql::get_partition_key(&session_token.trader_id);
                    let rk = ClientSessionNosql::get_row_key(&session_token.session_id);
                    let session = self.sessions_reader.get_entity(pk, &rk).await;

                    if session.is_none() {
                        return Err(AuthenticationFailedApiResponse::new(
                            ApiResultStatus::AccessTokenInvalid,
                            "Session not found".to_string(),
                        ));
                    }

                    let now = DateTimeAsMicroseconds::now();

                    if now.unix_microseconds >= session_token.get_expires_microseconds() {
                        return Err(AuthenticationFailedApiResponse::new(
                            ApiResultStatus::AccessTokenExpired,
                            "AccessToken expired".to_string(),
                        ));
                    }

                    let brand_id = session_token.get_brand_id().to_string();
                    ctx.request
                        .set_key_value(KV_BRAND_ID.to_string(), brand_id.into_bytes());

                    let session_id = session_token.get_session_id().to_owned();
                    ctx.request
                        .set_key_value(KV_SESSION_ID.to_string(), session_id.into_bytes());

                    ctx.credentials = Some(Box::new(session_token));

                    get_next.next(ctx).await
                } else {
                    Err(AuthenticationFailedApiResponse::new(
                        ApiResultStatus::AccessTokenInvalid,
                        "AccessToken invalid".to_string(),
                    ))
                }
            }
            None => {
                Err(AuthenticationFailedApiResponse::new(
                    ApiResultStatus::AccessTokenInvalid,
                    "AccessToken not found".to_string(),
                ))
            }
        };
    }
}

fn extract_token(src: &[u8]) -> &[u8] {
    let split_index = 6;

    if src.len() > split_index && src[split_index] == b' ' {
        return &src[7..];
    }

    src
}

#[cfg(test)]
mod tests {
    use crate::middlewares::auth_middleware::extract_token;

    #[test]
    fn test_extract_token() {
        let src = b"Bearer cCmSsx4q25RjB1kaggNu2/NhFsUxakLqjHE0tdpwmnySfceBKV8m86I7DRp5lf0yxln/W7Tqb0v29uRMDzWBdgLAdrR4IqmzD6lXLG5JwoNZKGlXRFvpyb7eg1UDFc7hg4+P0T7ijdYSX1XhbsR4x679R14yYZu/nN9foTempW0=";
        let result = extract_token(src);
        assert_eq!(result, b"cCmSsx4q25RjB1kaggNu2/NhFsUxakLqjHE0tdpwmnySfceBKV8m86I7DRp5lf0yxln/W7Tqb0v29uRMDzWBdgLAdrR4IqmzD6lXLG5JwoNZKGlXRFvpyb7eg1UDFc7hg4+P0T7ijdYSX1XhbsR4x679R14yYZu/nN9foTempW0=");
    }

    #[test]
    fn test_extract_token_without_bearer() {
        let src = b"cCmSsx4q25RjB1kaggNu2/NhFsUxakLqjHE0tdpwmnySfceBKV8m86I7DRp5lf0yxln/W7Tqb0v29uRMDzWBdgLAdrR4IqmzD6lXLG5JwoNZKGlXRFvpyb7eg1UDFc7hg4+P0T7ijdYSX1XhbsR4x679R14yYZu/nN9foTempW0=";
        let result = extract_token(src);
        assert_eq!(result, b"cCmSsx4q25RjB1kaggNu2/NhFsUxakLqjHE0tdpwmnySfceBKV8m86I7DRp5lf0yxln/W7Tqb0v29uRMDzWBdgLAdrR4IqmzD6lXLG5JwoNZKGlXRFvpyb7eg1UDFc7hg4+P0T7ijdYSX1XhbsR4x679R14yYZu/nN9foTempW0=");
    }
}
