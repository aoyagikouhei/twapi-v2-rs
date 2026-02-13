use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_tweets_id};
use twapi_v2::oauth10a::OAuthAuthentication;

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX cargo test test_get_2_tweets_id_oauth --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_id_oauth() -> Result<()> {
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
        let (res, _rate_limit) = execute_twitter::<serde_json::Value>(|| get_2_tweets_id::Api::open("1683460166149357568").build(&auth), &None).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_id::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
