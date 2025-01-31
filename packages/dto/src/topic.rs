use crate::*;
use by_macros::api_model;
// use num_format::{Locale, ToFormattedString};

#[cfg(feature = "server")]
use by_axum::aide;
use chrono::Datelike;

#[api_model(base = "/topics/v1", read_action = [get_topic] , table = topics, iter_type=QueryResponse)]
pub struct Topic {
    #[api_model(summary, primary_key)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub content: String,

    #[api_model(summary, action = create, action_by_id = update)]
    pub started_at: i64,
    // The end time of the vote
    #[api_model(summary, action = create, action_by_id = update)]
    pub ended_at: i64,
    // The number of required votes
    #[api_model(summary, action = create, action_by_id = update)]
    pub requirement: i64,
    // The number of voters
    #[api_model(summary, one_to_many = votes, foreign_key = topic_id, aggregator = count)]
    pub voters: i64,
    // // Donation amount
    #[api_model(summary, one_to_many = votes, foreign_key = topic_id, aggregator = sum(amount))]
    pub amount: i64,

    #[api_model(summary, many_to_many = votes, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = topic_id, aggregator = exist, unique)]
    pub voted: bool,
}

impl Topic {
    pub fn day(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!("{:02}", start.day())
    }

    pub fn month(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        match start.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Unknown",
        }
        .to_string()
    }

    pub fn date(&self) -> String {
        format!("{}/{}", self.month(), self.day())
    }

    pub fn period(&self) -> String {
        // to "12/15 - 1/22"
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();
        let end = chrono::DateTime::from_timestamp(self.ended_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!(
            "{:02}/{:02} - {:02}/{:02}",
            start.month(),
            start.day(),
            end.month(),
            end.day()
        )
    }

    // pub fn amount(&self) -> String {
    //     self.amount.to_formatted_string(&Locale::en)
    // }
}

impl TopicSummary {
    pub fn day(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!("{:02}", start.day())
    }

    pub fn month(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        match start.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Unknown",
        }
        .to_string()
    }

    pub fn date(&self) -> String {
        format!("{}/{}", self.month(), self.day())
    }

    pub fn period(&self) -> String {
        // to "12/15 - 1/22"
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();
        let end = chrono::DateTime::from_timestamp(self.ended_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!(
            "{:02}/{:02} - {:02}/{:02}",
            start.month(),
            start.day(),
            end.month(),
            end.day()
        )
    }

    // pub fn amount(&self) -> String {
    //     self.amount.to_formatted_string(&Locale::en)
    // }
}
