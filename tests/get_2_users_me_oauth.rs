use anyhow::Result;
use twapi_v2::{
    api::{execute_twitter, get_2_users_me},
    oauth10a::OAuthAuthentication,
};

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX cargo test test_get_2_users_me_oauth --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_me_oauth() -> Result<()> {
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let builder = get_2_users_me::Api::all().build(&auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_users_me::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
