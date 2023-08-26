use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_tweets_search_all, BearerAuth};

// ACADEMIC_BEARER_CODE=XXXXX cargo test test_get_2_tweets_search_all -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_search_all() -> Result<()> {
    let academic_bearer_code: String = match std::env::var("ACADEMIC_BEARER_CODE") {
        Ok(academic_bearer_code) => academic_bearer_code,
        _ => return Ok(()),
    };
    let bearer_auth = BearerAuth::new(academic_bearer_code);
    let builder = get_2_tweets_search_all::Api::open("東京")
        .max_results(10)
        .build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_search_all::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
