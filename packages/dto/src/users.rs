#![allow(unused)]
use crate::*;
use by_macros::*;

#[cfg(feature = "server")]
use by_axum::aide;
use lazy_static::lazy_static;
use validator::ValidationError;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", read_action = user_info, table = users)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: u64,
    #[api_model(auto = [insert, update])]
    pub updated_at: u64,

    #[api_model(action = signup)]
    pub nickname: String,
    #[api_model(action = signup, read_action = [check_email], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = signup)]
    pub profile_url: String,

    #[api_model(type = INTEGER)]
    pub role: UserRole,

    #[api_model(version = v0.1, unique, action = [login], read_action = [find_by_kakao_id])]
    pub kakao_id: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, ApiModel)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UserRole {
    Admin = 0,
    #[default]
    User = 1,
    // It means the user is not signed in web page.
    Guest = 10,
}

// fn validate_nickname(nickname: &str) -> std::result::Result<(), ValidationError> {
//     lazy_static! {
//         static ref NICKNAME_REGEX: regex::Regex =
//             regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-_]{1,20}$").unwrap();
//     }

//     if !NICKNAME_REGEX.is_match(nickname) {
//         return Err(ValidationError::new("Nickname must be started with alphabet or number and only allow alphabet, number, hyphen and underscore, maximum 20 characters"));
//     }

//     Ok(())
// }
