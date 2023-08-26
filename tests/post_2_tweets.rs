use anyhow::Result;
use chrono::prelude::*;
use twapi_v2::api::{execute_twitter, post_2_tweets, BearerAuthentication};

// BEARER_CODE=XXXXX TWEET_TEXT=hello cargo test test_post_2_tweets -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_tweets() -> Result<()> {
    let tweet_text = match std::env::var("TWEET_TEXT") {
        Ok(tweet_text) => tweet_text,
        _ => return Ok(()),
    };
    let now = Utc::now();
    let body = post_2_tweets::Body {
        text: Some(format!("now! {}, {}", now, tweet_text)),
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let builder = post_2_tweets::Api::new(body).build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
