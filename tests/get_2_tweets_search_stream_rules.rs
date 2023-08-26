use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_tweets_search_stream_rules, BearerAuth};

// APP_BEARER_CODE=XXXXX cargo test test_get_2_tweets_search_stream_rules -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_search_stream_rules() -> Result<()> {
    let app_bearer_code = match std::env::var("APP_BEARER_CODE") {
        Ok(app_bearer_code) => app_bearer_code,
        _ => return Ok(()),
    };
    let bearer_auth = BearerAuth::new(app_bearer_code);
    let builder = get_2_tweets_search_stream_rules::Api::new().build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_search_stream_rules::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
