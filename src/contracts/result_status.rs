use serde_repr::*;
use service_sdk::my_http_server;
use service_sdk::my_http_server::macros::MyHttpIntegerEnum;

#[derive(Serialize_repr, Deserialize_repr, MyHttpIntegerEnum, Debug)]
#[repr(i16)]
pub enum ApiResultStatus {
    #[http_enum_case(id=0; description="Operations was successful")]
    Ok,

    #[http_enum_case(id=-1; description="Invalid username or password")]
    InvalidUserNameOrPassword = -1,

    #[http_enum_case(id=-2; description="User exists")]
    UserExists = -2,

    #[http_enum_case(id=-3; description="User not found")]
    UserNotFound = -3,

    #[http_enum_case(id=-4 description="Old password is wrong")]
    OldPasswordIsWrong = -4,

    #[http_enum_case(id=-5; description="Wrong file extension")]
    WrongFileExtension = -5,

    #[http_enum_case(id=-6; description="File not found")]
    FileNotFound = -6,

    #[http_enum_case(id=-7; description="Personal data is not valid")]
    PersonalDataNotValid = -7,

    #[http_enum_case(id=-8; description="System error")]
    SystemError = -8,

    #[http_enum_case(id=-9; description="AccessTokenExpired")]
    AccessTokenExpired = -9,

    #[http_enum_case(id=-10; description="TechnicalError")]
    TechnicalError = -10,

    #[http_enum_case(id=-11; description="CountryRestriction")]
    CountryIsRestricted = -11,

    #[http_enum_case(id=-16; description="GoogleCodeRequired")]
    GoogleCodeRequired = -16,

    #[http_enum_case(id=-17; description="AccessTokenInvalid")]
    AccessTokenInvalid = -17,

    #[http_enum_case(id=-18; description="AccessClaimRequired")]
    AccessClaimRequired = -18,

    #[http_enum_case(id=-19; description="OperationBlocked")]
    OperationBlocked = -19,

    #[http_enum_case(id=-20; description="Failed validation for MinInvestAmount")]
    MinInvestAmountInvalid = -20,

    #[http_enum_case(id=-21; description="Failed validation for MaxCumulativeInvestVolumeInvalid")]
    MaxCumulativeInvestVolumeInvalid = -21,

    #[http_enum_case(id=-21; description="EmailNotAllowed")]
    EmailNotAllowed = -22,

    #[http_enum_case(id=-999; description="Force Update required")]
    ForceUpdateIsRequired = -999,
}

#[cfg(test)]
mod test {
    use super::ApiResultStatus;
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct TestStruct {
        result: ApiResultStatus,
    }

    #[test]
    pub fn test_reult_deserialization() {
        let test_struct = TestStruct {
            result: ApiResultStatus::AccessTokenExpired,
        };

        let result = serde_json::to_string(&test_struct).unwrap();

        println!("{}", result);
    }
}
