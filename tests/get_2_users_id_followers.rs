use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_users_id_followers};

// BEARER_CODE=XXXXX cargo test test_get_2_users_id_followers -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_id_followers() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder =
        get_2_users_id_followers::Api::open(&bearer_code, "19522946").build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_users_id_followers::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);

    if let Some(next_token) = response.meta.unwrap().next_token {
        let builder = get_2_users_id_followers::Api::open(&bearer_code, "19522946")
            .pagination_token(&next_token)
            .build();
        let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
        println!("\n{}", serde_json::to_string(&res).unwrap());
        let response = serde_json::from_value::<get_2_users_id_followers::Response>(res)?;
        assert_eq!(response.is_empty_extra(), true);
    }

    Ok(())
}
