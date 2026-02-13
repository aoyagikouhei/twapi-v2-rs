use anyhow::Result;
use twapi_v2::api::{self, BearerAuthentication, execute_twitter, get_2_usage_tweets};

// BEARER_CODE=XXXX cargo test test_get_2_usage_tweets_oauth --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_usage_tweets_oauth() -> Result<()> {
    api::clear_prefix_url();
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let (res, _) = execute_twitter::<serde_json::Value>(|| get_2_usage_tweets::Api::all().build(&bearer_auth), &None).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_usage_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
