use anyhow::Result;
use chrono::prelude::*;
use twapi_v2::api::{execute_twitter, post_2_dm_conversations::{self, Message}};

// BEARER_CODE=XXXXX PARTICIPANT_IDS=XXXXX,YYYY cargo test test_post_2_dm_conversations -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_dm_conversations() -> Result<()> {
    let participant_ids = match std::env::var("PARTICIPANT_IDS") {
        Ok(participant_ids) => participant_ids.split(",").collect::<Vec<&str>>().into_iter().map(|it| it.to_owned()).collect::<Vec<String>>(),
        _ => return Ok(()),
    };
    let now = Utc::now();
    let body = post_2_dm_conversations::Body {
        conversation_type: post_2_dm_conversations::ConversationType::Group,
        message: Message {
            text: Some(format!("now! {}", now)),
            ..Default::default()
        },
        participant_ids,
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = post_2_dm_conversations::Api::new(
        &bearer_code,
        body,
    )
    .build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<
        post_2_dm_conversations::Response,
    >(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
