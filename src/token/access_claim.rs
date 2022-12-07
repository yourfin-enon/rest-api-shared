use std::fmt;
use my_http_server_swagger::MyHttpIntegerEnum;
use serde_repr::{Serialize_repr,Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, MyHttpIntegerEnum, Debug)]
#[repr(i16)]
pub enum AccessClaimType {
    #[http_enum_case(id="0"; description="EmailConfirmed")]
    EmailConfirmed,
}

impl fmt::Display for AccessClaimType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessClaimType::EmailConfirmed => write!(f, "EmailConfirmed"),
        }
    }
}