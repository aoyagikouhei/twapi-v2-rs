use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, execute_twitter, get_2_users_me};

//BEARER_CODE=XXXXX cargo test --test get_2_users_me --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_me() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let (res, rate_limit) = execute_twitter::<serde_json::Value>(
        || get_2_users_me::Api::all().build(&bearer_auth),
        &None,
    )
    .await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    println!("{}", rate_limit);
    let response = serde_json::from_value::<get_2_users_me::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
