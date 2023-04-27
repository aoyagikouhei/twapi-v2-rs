use anyhow::Result;
use twapi_v2::api::{delete_2_users_id_retweets_source_tweet_id, execute_twitter};

// BEARER_CODE=XXXXX ME_ID=XXXX TWEET_ID=XXXXX cargo test test_delete_2_users_id_retweets_source_tweet_id -- --nocapture --test-threads=1

#[tokio::test]
async fn test_delete_2_users_id_retweets_source_tweet_id() -> Result<()> {
    let me_id = match std::env::var("ME_ID") {
        Ok(me_id) => me_id,
        _ => return Ok(()),
    };
    let tweet_id = match std::env::var("TWEET_ID") {
        Ok(tweet_id) => tweet_id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder =
        delete_2_users_id_retweets_source_tweet_id::Api::new(&bearer_code, &me_id, &tweet_id)
            .build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response =
        serde_json::from_value::<delete_2_users_id_retweets_source_tweet_id::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
