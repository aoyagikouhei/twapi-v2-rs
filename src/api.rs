use crate::{
    error::Error,
    error::{FieldError, TwitterError},
    rate_limit::RateLimit,
};

pub mod delete_2_tweets;
pub mod get_2_dm_conversations_dm_conversation_id_dm_events;
pub mod get_2_dm_conversations_with_participant_id_dm_events;
pub mod get_2_dm_events;
pub mod get_2_tweets_id_liking_users;
pub mod get_2_tweets_id_retweeted_by;
pub mod get_2_tweets_id;
pub mod get_2_tweets_search_recent;
pub mod get_2_tweets_search_stream_rules;
pub mod get_2_tweets_search_stream;
pub mod get_2_tweets;
pub mod get_2_users_id_mentions;
pub mod get_2_users_id_timelines_reverse_chronological;
pub mod get_2_users_id_tweets;
pub mod get_2_users_me;
pub mod post_2_dm_conversations;
pub mod post_2_dm_conversations_dm_conversation_id_message;
pub mod post_2_dm_conversations_with_participant_id_message;
pub mod post_2_oauth2_token;
pub mod post_2_tweets;
pub mod post_2_tweets_search_stream_rules;

pub type TwitterResult = Result<(serde_json::Value, Option<RateLimit>), Error>;

pub async fn execute_twitter(builder: reqwest::RequestBuilder) -> TwitterResult {
    let response = builder.send().await?;
    let status_code = response.status();
    let header = response.headers();
    let rate_limit = RateLimit::new(header);

    let value: serde_json::Value = response.json().await?;

    if value["errors"].is_array() {
        Err(FieldError::field_error(status_code, value, rate_limit))
    } else if status_code.is_success() {
        Ok((value, rate_limit))
    } else {
        let res = TwitterError::new(status_code, value);
        Err(Error::Twitter(res, rate_limit))
    }
}
