use anyhow::Result;
use twapi_v2::api::{execute_twitter, put_2_tweets_id_hidden};

// BEARER_CODE=XXXXX cargo test test_put_2_tweets_id_hidden -- --nocapture --test-threads=1

#[tokio::test]
async fn test_put_2_tweets_id_hidden() -> Result<()> {
    let body = put_2_tweets_id_hidden::Body { hidden: true };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder =
        put_2_tweets_id_hidden::Api::new(&bearer_code, "1653503816095133696", body).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<put_2_tweets_id_hidden::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
