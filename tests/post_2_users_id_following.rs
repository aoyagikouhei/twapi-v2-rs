use anyhow::Result;
use twapi_v2::api::{execute_twitter, post_2_users_id_following};

// BEARER_CODE=XXXXX TARGET_USER_ID=XXXXX cargo test test_post_2_users_id_following -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_users_id_following() -> Result<()> {
    let target_user_id = match std::env::var("TARGET_USER_ID") {
        Ok(target_user_id) => target_user_id,
        _ => return Ok(()),
    };
    let body = post_2_users_id_following::Body { target_user_id };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder =
        post_2_users_id_following::Api::new(&bearer_code, "1660518823991336966", body).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_users_id_following::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
