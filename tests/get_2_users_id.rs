use anyhow::Result;
use twapi_v2::{api::{BearerAuthentication, execute_twitter, get_2_users_id}, fields::user_fields::UserFields};

// BEARER_CODE=XXXXX cargo test test_get_2_users_id --  --all-features --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_id() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
        let (res, _rate_limit) = execute_twitter::<serde_json::Value>(|| get_2_users_id::Api::open("1660518823991336966")
    .user_fields(UserFields::all())
    .build(&bearer_auth)).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_users_id::Response>(res)?;
    let data = response.data.as_ref().unwrap();
    assert_eq!(data.username, "barley_tea_0522");
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
