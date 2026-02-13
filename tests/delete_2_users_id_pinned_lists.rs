use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, delete_2_users_id_pinned_lists, execute_twitter};

// BEARER_CODE=XXXXX LIST_ID=XXXXX cargo test test_delete_2_users_id_pinned_lists -- --nocapture --test-threads=1

#[tokio::test]
async fn test_delete_2_users_id_pinned_lists() -> Result<()> {
    let list_id = match std::env::var("LIST_ID") {
        Ok(list_id) => list_id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
        let (res, _rate_limit) = execute_twitter::<serde_json::Value>(|| delete_2_users_id_pinned_lists::Api::new("1660518823991336966", &list_id)
        .build(&bearer_auth), &None).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<delete_2_users_id_pinned_lists::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
