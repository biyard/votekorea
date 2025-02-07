#![allow(unused_imports)]
use crate::*;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

// If you want to know how to use Y macro, refer to https://github.com/biyard/rust-sdk/tree/main/packages/by-macros
#[api_model(base = "/v1/topics/:topic-id/votes", table = votes)]
pub struct Vote {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = topics)]
    pub topic_id: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, action = support)]
    pub amount: i64,

    #[api_model(summary, action = support)]
    pub name: String,

    #[api_model(action_by_id = confirm)]
    pub confirmed: Option<bool>,
}
