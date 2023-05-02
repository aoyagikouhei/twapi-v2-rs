use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_spaces_id_buyers};

// BEARER_CODE=XXXXX SPACES_ID=XXXXX cargo test test_get_2_spaces_id_buyers -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_spaces_id_buyers() -> Result<()> {
    let spaces_id = match std::env::var("SPACES_ID") {
        Ok(spaces_id) => spaces_id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = get_2_spaces_id_buyers::Api::all(&bearer_code, &spaces_id).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_spaces_id_buyers::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
