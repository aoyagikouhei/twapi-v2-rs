use anyhow::Result;
use chrono::prelude::*;
use twapi_v2::api::{
    BearerAuthentication, execute_twitter, post_2_dm_conversations_dm_conversation_id_message,
};

// BEARER_CODE=XXXXX DM_CONVERSATION_ID=XXXXX cargo test test_post_2_dm_conversations_dm_conversation_id_message -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_dm_conversations_dm_conversation_id_message() -> Result<()> {
    let dm_conversation_id = match std::env::var("DM_CONVERSATION_ID") {
        Ok(dm_conversation_id) => dm_conversation_id,
        _ => return Ok(()),
    };
    let now = Utc::now();
    let body = post_2_dm_conversations_dm_conversation_id_message::Body {
        text: Some(format!("now! {}", now)),
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let builder =
        post_2_dm_conversations_dm_conversation_id_message::Api::new(&dm_conversation_id, body)
            .build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<
        post_2_dm_conversations_dm_conversation_id_message::Response,
    >(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
