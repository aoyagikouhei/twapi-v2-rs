use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, execute_twitter, post_2_users_id_likes};

// BEARER_CODE=XXXXX TWEET_ID=XXXXX cargo test test_post_2_users_id_likes -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_users_id_likes() -> Result<()> {
    let tweet_id = match std::env::var("TWEET_ID") {
        Ok(tweet_id) => tweet_id,
        _ => return Ok(()),
    };
    let body = post_2_users_id_likes::Body { tweet_id };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(
        || post_2_users_id_likes::Api::new("1660518823991336966", body.clone()).build(&bearer_auth),
        &None,
    )
    .await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_users_id_likes::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
