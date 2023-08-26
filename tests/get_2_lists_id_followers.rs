use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_lists_id_followers, BearerAuth};

// BEARER_CODE=XXXXX cargo test test_get_2_lists_id_followers -- --nocapture --test-threads=1

#[tokio::test]
#[ignore]
async fn test_get_2_lists_id_followers() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuth::new(bearer_code);
    let builder = get_2_lists_id_followers::Api::open("1686145482224254977").build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_lists_id_followers::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
