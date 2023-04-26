use anyhow::Result;
use chrono::prelude::*;
use twapi_v2::api::{execute_twitter, post_2_tweets};

// BEARER_CODE=XXXXX cargo test test_post_2_tweets -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_tweets() -> Result<()> {
    let now = Utc::now();
    let body = post_2_tweets::Body {
        text: Some(format!("now! {}", now)),
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = post_2_tweets::Api::new(&bearer_code, body).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
