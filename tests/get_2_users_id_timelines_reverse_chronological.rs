use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_users_id_timelines_reverse_chronological};

// BEARER_CODE=XXXXX cargo test test_get_2_users_id_timelines_reverse_chronological -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_id_timelines_reverse_chronological() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = get_2_users_id_timelines_reverse_chronological::Api::open(
        &bearer_code,
        "1660518823991336966",
    )
    .build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response =
        serde_json::from_value::<get_2_users_id_timelines_reverse_chronological::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
