use anyhow::Result;
use twapi_v2::api::{
    BearerAuthentication, execute_twitter, get_2_dm_conversations_with_participant_id_dm_events,
};

// BEARER_CODE=XXXXX PARTICIPANT_ID=XXXXX cargo test test_get_2_dm_conversations_with_participant_id_dm_events -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_dm_conversations_with_participant_id_dm_events() -> Result<()> {
    let participant_id = match std::env::var("PARTICIPANT_ID") {
        Ok(participant_id) => participant_id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(
        || {
            get_2_dm_conversations_with_participant_id_dm_events::Api::all(&participant_id)
                .build(&bearer_auth)
        },
        &None,
    )
    .await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<
        get_2_dm_conversations_with_participant_id_dm_events::Response,
    >(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
