use anyhow::Result;
use twapi_v2::{
    api::{BearerAuthentication, execute_twitter, get_2_spaces_id},
    fields::space_fields::SpaceFields,
};

// BEARER_CODE=XXXXX SPACES_ID=XXXXX cargo test test_get_2_spaces_id -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_spaces_id() -> Result<()> {
    let id = match std::env::var("SPACES_ID") {
        Ok(id) => id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let mut expantions = get_2_spaces_id::Expansions::all();
    // Setting this paramter is invalid.
    expantions.remove(&get_2_spaces_id::Expansions::TopicsIds);
    let mut spaces_fields = SpaceFields::all();
    // Setting this paramter is invalid.
    spaces_fields.remove(&SpaceFields::SubscriberCount);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(
        || {
            get_2_spaces_id::Api::all(&id)
                .space_fields(spaces_fields.clone())
                .expansions(expantions.clone())
                .build(&bearer_auth)
        },
        &None,
    )
    .await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_spaces_id::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
