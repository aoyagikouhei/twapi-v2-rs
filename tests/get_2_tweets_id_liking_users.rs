use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_tweets_id_liking_users};

// BEARER_CODE=XXXXX cargo test test_get_2_tweets_id_liking_users -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_id_liking_users() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder =
        get_2_tweets_id_liking_users::Api::open(&bearer_code, "1617696866413719556").build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_id_liking_users::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);

    /*
    let builder = get_2_tweets_id_liking_users::Api::open(&bearer_code, "1617696866413719556")
        .pagination_token(&response.meta.unwrap().next_token.unwrap())
        .build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("\n{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_id_liking_users::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
     */

    Ok(())
}
