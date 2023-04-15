use crate::{error::Error, error::TwitterError, rate_limit::RateLimit};

pub mod get_2_tweets_id_liking_users;
pub mod get_2_tweets_search_recent;

pub type TwitterResult = Result<(serde_json::Value, Option<RateLimit>), Error>;

pub async fn execute_twitter(builder: reqwest::RequestBuilder) -> TwitterResult {
    let response = builder.send().await?;
    let status_code = response.status();
    let header = response.headers();
    let rate_limit = RateLimit::new(header);

    let value: serde_json::Value = response.json().await?;
    if status_code.is_success() {
        Ok((value, rate_limit))
    } else {
        let res = TwitterError::new(status_code, value);
        Err(Error::Twitter(res, rate_limit))
    }
}
