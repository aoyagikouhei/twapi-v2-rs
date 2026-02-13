use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, execute_twitter, get_2_spaces_by_creator_ids};

// BEARER_CODE=XXXXX USER_IDS=XXXXX cargo test test_get_2_spaces_by_creator_ids -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_spaces_by_creator_ids() -> Result<()> {
    let user_ids = match std::env::var("USER_IDS") {
        Ok(user_ids) => user_ids,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let mut expantions = get_2_spaces_by_creator_ids::Expansions::all();
    // Setting this paramter is invalid.
    expantions.remove(&get_2_spaces_by_creator_ids::Expansions::TopicsIds);
        let (res, _rate_limit) = execute_twitter::<serde_json::Value>(|| get_2_spaces_by_creator_ids::Api::all(&user_ids)
        .expansions(expantions.clone())
        .build(&bearer_auth), &None).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_spaces_by_creator_ids::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
