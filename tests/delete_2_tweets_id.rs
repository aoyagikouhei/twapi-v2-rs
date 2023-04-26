use anyhow::Result;
use twapi_v2::api::{execute_twitter, delete_2_tweets_id};

// BEARER_CODE=XXXXX DELETE_TWEET_ID=XXXXX cargo test test_delete_2_tweets_id -- --nocapture --test-threads=1

#[tokio::test]
async fn test_delete_2_tweets_id() -> Result<()> {
    let tweet_id = match std::env::var("DELETE_TWEET_ID") {
        Ok(tweet_id) => tweet_id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = delete_2_tweets_id::Api::new(&bearer_code, &tweet_id).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<delete_2_tweets_id::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
