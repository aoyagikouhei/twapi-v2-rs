use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, execute_twitter, get_2_tweets_count_all};

// ACADEMIC_BEARER_CODE=XXXXX cargo test test_get_2_tweets_count_all -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_count_all() -> Result<()> {
    let academic_bearer_code: String = match std::env::var("ACADEMIC_BEARER_CODE") {
        Ok(academic_bearer_code) => academic_bearer_code,
        _ => return Ok(()),
    };
    let bearer_auth = BearerAuthentication::new(academic_bearer_code);
        let (res, _rate_limit) = execute_twitter::<serde_json::Value>(|| get_2_tweets_count_all::Api::new("東京").build(&bearer_auth)).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_count_all::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
