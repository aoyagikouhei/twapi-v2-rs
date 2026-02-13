use std::time::Duration;

use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use crate::{
    error::{Error, TwitterError},
    headers::Headers,
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
pub mod get_2_media_upload;
pub mod get_2_spaces;
pub mod get_2_spaces_by_creator_ids;
pub mod get_2_spaces_id;
pub mod get_2_spaces_id_buyers;
pub mod get_2_spaces_id_tweets;
pub mod get_2_spaces_search;
pub mod get_2_trends_by_woeid_woeid;
pub mod get_2_tweets;
pub mod get_2_tweets_compliance_stream;
pub mod get_2_tweets_count_all;
pub mod get_2_tweets_count_recent;
pub mod get_2_tweets_id;
pub mod get_2_tweets_id_liking_users;
pub mod get_2_tweets_id_quote_tweets;
pub mod get_2_tweets_id_retweeted_by;
pub mod get_2_tweets_id_retweets;
pub mod get_2_tweets_sample10_stream;
pub mod get_2_tweets_sample_stream;
pub mod get_2_tweets_search_all;
pub mod get_2_tweets_search_recent;
pub mod get_2_tweets_search_stream;
pub mod get_2_tweets_search_stream_rules;
pub mod get_2_usage_tweets;
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
pub mod get_2_users_reposts_of_me;
pub mod get_2_users_search;
pub mod post_2_compliance_jobs;
pub mod post_2_dm_conversations;
pub mod post_2_dm_conversations_dm_conversation_id_message;
pub mod post_2_dm_conversations_with_participant_id_message;
pub mod post_2_lists;
pub mod post_2_lists_id_members;
pub mod post_2_media_metadata_create;
pub mod post_2_media_subtitles_create;
pub mod post_2_media_subtitles_delete;
pub mod post_2_media_upload_id_append;
pub mod post_2_media_upload_id_finalize;
pub mod post_2_media_upload_initialize;
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

const ENV_KEY: &str = "TWAPI_V2_TWITTER_API_PREFIX_API";
const PREFIX_URL_TWITTER: &str = "https://api.x.com";

pub fn clear_prefix_url() {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { std::env::set_var(ENV_KEY, PREFIX_URL_TWITTER) };
}

pub fn setup_prefix_url(url: &str) {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { std::env::set_var(ENV_KEY, url) };
}

#[derive(Debug, Clone, Default)]
pub struct TwapiOptions {
    pub prefix_url: Option<String>,
    pub timeout: Option<Duration>,
}

pub(crate) fn make_url(twapi_options: &Option<TwapiOptions>, post_url: &str) -> String {
    make_url_with_prefix(
        &std::env::var(ENV_KEY).unwrap_or(PREFIX_URL_TWITTER.to_owned()),
        twapi_options,
        post_url,
    )
}

pub(crate) fn make_url_with_prefix(
    default_perfix_url: &str,
    twapi_options: &Option<TwapiOptions>,
    post_url: &str,
) -> String {
    let prefix_url = if let Some(twapi_options) = twapi_options {
        if let Some(prefix_url) = twapi_options.prefix_url.as_ref() {
            prefix_url
        } else {
            default_perfix_url
        }
    } else {
        default_perfix_url
    };
    format!("{}{}", prefix_url, post_url)
}

pub trait Authentication {
    fn execute(
        &self,
        builder: RequestBuilder,
        method: &str,
        uri: &str,
        options: &[(&str, &str)],
    ) -> RequestBuilder;
}

pub struct BearerAuthentication {
    bearer_code: String,
}

impl BearerAuthentication {
    pub fn new<T: Into<String>>(bearer_code: T) -> Self {
        Self {
            bearer_code: bearer_code.into(),
        }
    }
}

impl Authentication for BearerAuthentication {
    fn execute(
        &self,
        builder: RequestBuilder,
        _method: &str,
        _uri: &str,
        _options: &[(&str, &str)],
    ) -> RequestBuilder {
        builder.bearer_auth(&self.bearer_code)
    }
}

pub async fn execute_twitter<T>(f: impl Fn() -> RequestBuilder,) -> Result<(T, Headers), Error>
where
    T: DeserializeOwned,
{
    let response = f().send().await?;
    let status_code = response.status();
    let header = response.headers();
    let headers = Headers::new(header);

    println!("status_code: {:?}", status_code);

    if status_code.is_success() {
        Ok((response.json::<T>().await?, headers))
    } else {
        let text = response.text().await?;
        println!("text: {:?}", text);
        match serde_json::from_str(&text) {
            Ok(value) => Err(Error::Twitter(
                TwitterError::new(&value, status_code),
                value,
                Box::new(headers),
            )),
            Err(err) => Err(Error::Other(
                format!("{:?}:{}", err, text),
                Some(status_code),
            )),
        }
    }
}

pub(crate) fn apply_options(
    client: RequestBuilder,
    options: &Option<TwapiOptions>,
) -> RequestBuilder {
    let Some(options) = options else {
        return client;
    };
    let Some(timeout) = options.timeout else {
        return client;
    };
    client.timeout(timeout)
}
