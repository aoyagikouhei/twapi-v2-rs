use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, execute_twitter, get_2_tweets};

//BEARER_CODE=XXXXX cargo test --test get_2_tweets --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_tweets() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let builder =
        get_2_tweets::Api::all("1648835862149619712,1653186227338874881").build(&bearer_auth);
    let (res, rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    println!("{}", rate_limit);
    let response = serde_json::from_value::<get_2_tweets::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
