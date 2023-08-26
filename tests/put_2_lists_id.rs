use anyhow::Result;
use twapi_v2::api::{execute_twitter, put_2_lists_id, BearerAuth};

// BEARER_CODE=XXXXX LIST_ID=XXXXX cargo test test_put_2_lists_id -- --nocapture --test-threads=1

#[tokio::test]
async fn test_put_2_lists_id() -> Result<()> {
    let list_id = match std::env::var("LIST_ID") {
        Ok(list_id) => list_id,
        _ => return Ok(()),
    };
    let body = put_2_lists_id::Body {
        name: Some("test2".to_owned()),
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuth::new(bearer_code);
    let builder = put_2_lists_id::Api::new(&list_id, body).build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<put_2_lists_id::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
