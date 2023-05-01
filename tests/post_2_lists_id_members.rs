use anyhow::Result;
use twapi_v2::api::{execute_twitter, post_2_lists_id_members};

// BEARER_CODE=XXXXX TARGET_USER_ID=XXXXX cargo test test_post_2_lists_id_members -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_lists_id_members() -> Result<()> {
    let user_id = match std::env::var("TARGET_USER_ID") {
        Ok(user_id) => user_id,
        _ => return Ok(()),
    };
    let body = post_2_lists_id_members::Body { user_id };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = post_2_lists_id_members::Api::new(&bearer_code, "14914404", body).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_lists_id_members::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
