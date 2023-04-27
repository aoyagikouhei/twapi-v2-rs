use anyhow::Result;
use chrono::prelude::*;
use twapi_v2::api::{execute_twitter, post_2_dm_conversations_with_participant_id_message};

// BEARER_CODE=XXXXX PARTICIPANT_ID=XXXXX cargo test test_post_2_dm_conversations_with_participant_id_message -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_dm_conversations_with_participant_id_message() -> Result<()> {
    let participant_id = match std::env::var("PARTICIPANT_ID") {
        Ok(participant_id) => participant_id,
        _ => return Ok(()),
    };
    let now = Utc::now();
    let body = post_2_dm_conversations_with_participant_id_message::Body {
        text: Some(format!("now! {}", now)),
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = post_2_dm_conversations_with_participant_id_message::Api::new(
        &bearer_code,
        &participant_id,
        body,
    )
    .build();
    println!("{:?}", builder);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<
        post_2_dm_conversations_with_participant_id_message::Response,
    >(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
