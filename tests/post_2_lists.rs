use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, execute_twitter, post_2_lists};

// BEARER_CODE=XXXXX LIST_NAME=XXXXX cargo test test_post_2_lists -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_lists() -> Result<()> {
    let name = match std::env::var("LIST_NAME") {
        Ok(name) => name,
        _ => return Ok(()),
    };
    let body = post_2_lists::Body {
        name,
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let builder = post_2_lists::Api::new(body).build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_lists::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
