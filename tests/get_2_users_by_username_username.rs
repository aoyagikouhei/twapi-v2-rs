use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_users_by_username_username};

// BEARER_CODE=XXXXX USER_NAME=XXXXX cargo test test_get_2_users_by_username_username -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_by_username_username() -> Result<()> {
    let user_name = match std::env::var("USER_NAME") {
        Ok(user_name) => user_name,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder =
        get_2_users_by_username_username::Api::open(&bearer_code, &user_name).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_users_by_username_username::Response>(res)?;
    let data = response.data.as_ref().unwrap();
    assert_eq!(data.username, user_name);
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
