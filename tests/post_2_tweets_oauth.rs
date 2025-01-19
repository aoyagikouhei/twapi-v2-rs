use anyhow::Result;
use chrono::prelude::*;
use twapi_v2::{
    api::{execute_twitter, post_2_tweets},
    oauth10a::OAuthAuthentication,
};

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX TWEET_TEXT=hello cargo test test_post_2_tweets_oauth -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_tweets_oauth() -> Result<()> {
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let tweet_text = match std::env::var("TWEET_TEXT") {
        Ok(tweet_text) => tweet_text,
        _ => return Ok(()),
    };
    let now = Utc::now();
    let body = post_2_tweets::Body {
        text: Some(format!("now! {}, {}", now, tweet_text)),
        ..Default::default()
    };
    let builder = post_2_tweets::Api::new(body).build(&auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);

    Ok(())
}
