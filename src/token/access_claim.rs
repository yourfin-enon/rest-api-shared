use std::fmt;
use serde_repr::{Serialize_repr,Deserialize_repr};
use service_sdk::my_http_server;
use service_sdk::my_http_server::macros::{MyHttpIntegerEnum};

#[derive(Serialize_repr, Deserialize_repr, MyHttpIntegerEnum, Debug)]
#[repr(i16)]
pub enum AccessClaimType {
    #[http_enum_case(id="0"; description="EmailConfirmed")]
    EmailConfirmed,
    #[http_enum_case(id="1"; description="LoginTwoFaConfirmed")]
    LoginTwoFaConfirmed,
    #[http_enum_case(id="2"; description="WithdrawalTwoFaConfirmed")]
    WithdrawalTwoFaConfirmed,
}

impl fmt::Display for AccessClaimType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessClaimType::EmailConfirmed => write!(f, "EmailConfirmed"),
            AccessClaimType::LoginTwoFaConfirmed => write!(f, "LoginTwoFaConfirmed"),
            AccessClaimType::WithdrawalTwoFaConfirmed => write!(f, "WithdrawalTwoFaConfirmed"),
        }
    }
}