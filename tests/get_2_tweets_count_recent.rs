use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_tweets_count_recent};

// APP_BEARER_CODE=XXXXX cargo test test_get_2_tweets_count_recent -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_count_recent() -> Result<()> {
    let app_bearer_code = match std::env::var("APP_BEARER_CODE") {
        Ok(app_bearer_code) => app_bearer_code,
        _ => return Ok(()),
    };
    let builder = get_2_tweets_count_recent::Api::new(&app_bearer_code, "東京").build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_count_recent::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
