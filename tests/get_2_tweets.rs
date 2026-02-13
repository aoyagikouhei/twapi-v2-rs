use std::time::Duration;

use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, TwapiOptions, execute_twitter, get_2_tweets};

//BEARER_CODE=XXXXX cargo test --test get_2_tweets --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_tweets() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let twapi_option = TwapiOptions {
        try_count: Some(4),
        retryable_status_codes: Some(vec![500, 502, 503, 504, 401]),
        retry_interval_duration: Some(Duration::from_millis(1000)),
        ..Default::default()
    };
    let (res, rate_limit) = execute_twitter::<serde_json::Value>(
        || get_2_tweets::Api::all("1,2").build(&bearer_auth),
        &Some(twapi_option),
    )
    .await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    println!("{}", rate_limit);
    let response = serde_json::from_value::<get_2_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
