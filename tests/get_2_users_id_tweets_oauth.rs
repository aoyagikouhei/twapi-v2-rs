use anyhow::Result;
use twapi_v2::{
    api::{execute_twitter, get_2_users_id_tweets},
    oauth10a::OAuthAuthentication,
};

// BEARER_CODE=XXXXX cargo test test_get_2_users_id_tweets_oauth --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_id_tweets_oauth() -> Result<()> {
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let (res, headers) = execute_twitter::<serde_json::Value>(|| get_2_users_id_tweets::Api::open("19522946").build(&auth)).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    println!("{}", headers);
    let response = serde_json::from_value::<get_2_users_id_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
