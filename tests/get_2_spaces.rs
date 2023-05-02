use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_spaces};

// BEARER_CODE=XXXXX SPACES_IDS=XXXXX cargo test test_get_2_spaces -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_spaces() -> Result<()> {
    let ids = match std::env::var("SPACES_IDS") {
        Ok(ids) => ids,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let mut expantions = get_2_spaces::Expansions::all();
    // Setting this paramter is invalid.
    expantions.remove(&get_2_spaces::Expansions::TopicsIds);
    let builder = get_2_spaces::Api::all(&bearer_code, &ids)
        .expansions(expantions)
        .build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_spaces::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
