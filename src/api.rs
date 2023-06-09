use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use crate::{
    error::{Error, TwitterError},
    rate_limit::RateLimit,
};

pub mod delete_2_lists_id;
pub mod delete_2_lists_id_members_user_id;
pub mod delete_2_tweets_id;
pub mod delete_2_users_id_bookmarks_tweet_id;
pub mod delete_2_users_id_followed_lists_list_id;
pub mod delete_2_users_id_likes_tweet_id;
pub mod delete_2_users_id_pinned_lists;
pub mod delete_2_users_id_retweets_source_tweet_id;
pub mod delete_2_users_source_user_id_blocking_target_user_id;
pub mod delete_2_users_source_user_id_following_target_user_id;
pub mod delete_2_users_source_user_id_muting_target_user_id;
pub mod get_2_compliance_jobs;
pub mod get_2_compliance_jobs_id;
pub mod get_2_dm_conversations_dm_conversation_id_dm_events;
pub mod get_2_dm_conversations_with_participant_id_dm_events;
pub mod get_2_dm_events;
pub mod get_2_lists_id;
pub mod get_2_lists_id_followers;
pub mod get_2_lists_id_members;
pub mod get_2_lists_id_tweets;
pub mod get_2_spaces;
pub mod get_2_spaces_by_creator_ids;
pub mod get_2_spaces_id;
pub mod get_2_spaces_id_buyers;
pub mod get_2_spaces_id_tweets;
pub mod get_2_spaces_search;
pub mod get_2_tweets;
pub mod get_2_tweets_compliance_stream;
pub mod get_2_tweets_count_all;
pub mod get_2_tweets_count_recent;
pub mod get_2_tweets_id;
pub mod get_2_tweets_id_liking_users;
pub mod get_2_tweets_id_quote_tweets;
pub mod get_2_tweets_id_retweeted_by;
pub mod get_2_tweets_sample10_stream;
pub mod get_2_tweets_sample_stream;
pub mod get_2_tweets_search_all;
pub mod get_2_tweets_search_recent;
pub mod get_2_tweets_search_stream;
pub mod get_2_tweets_search_stream_rules;
pub mod get_2_users;
pub mod get_2_users_by;
pub mod get_2_users_by_username_username;
pub mod get_2_users_compliance_stream;
pub mod get_2_users_id;
pub mod get_2_users_id_blocking;
pub mod get_2_users_id_bookmarks;
pub mod get_2_users_id_followed_lists;
pub mod get_2_users_id_followers;
pub mod get_2_users_id_following;
pub mod get_2_users_id_liked_tweets;
pub mod get_2_users_id_list_memberships;
pub mod get_2_users_id_mentions;
pub mod get_2_users_id_muting;
pub mod get_2_users_id_owned_lists;
pub mod get_2_users_id_pinned_lists;
pub mod get_2_users_id_timelines_reverse_chronological;
pub mod get_2_users_id_tweets;
pub mod get_2_users_me;
pub mod post_2_compliance_jobs;
pub mod post_2_dm_conversations;
pub mod post_2_dm_conversations_dm_conversation_id_message;
pub mod post_2_dm_conversations_with_participant_id_message;
pub mod post_2_lists;
pub mod post_2_lists_id_members;
pub mod post_2_oauth2_token_refresh_token;
pub mod post_2_tweets;
pub mod post_2_tweets_search_stream_rules;
pub mod post_2_users_id_blocking;
pub mod post_2_users_id_bookmarks;
pub mod post_2_users_id_followed_lists;
pub mod post_2_users_id_following;
pub mod post_2_users_id_likes;
pub mod post_2_users_id_muting;
pub mod post_2_users_id_pinned_lists;
pub mod post_2_users_id_retweets;
pub mod put_2_lists_id;
pub mod put_2_tweets_id_hidden;

pub async fn execute_twitter<T>(builder: RequestBuilder) -> Result<(T, Option<RateLimit>), Error>
where
    T: DeserializeOwned,
{
    let response = builder.send().await?;
    let status_code = response.status();
    let header = response.headers();
    let rate_limit = RateLimit::new(header);

    if status_code.is_success() {
        Ok((response.json::<T>().await?, rate_limit))
    } else {
        match response.json::<serde_json::Value>().await {
            Ok(value) => Err(Error::Twitter(
                TwitterError::new(&value, status_code),
                value,
                rate_limit,
            )),
            Err(err) => Err(Error::Other(format!("{:?}", err), Some(status_code))),
        }
    }
}
