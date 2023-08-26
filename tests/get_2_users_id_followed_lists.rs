use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_users_id_followed_lists, BearerAuth};

// BEARER_CODE=XXXXX cargo test test_get_2_users_id_followed_lists -- --nocapture --test-threads=1

#[tokio::test]
#[ignore]
async fn test_get_2_users_id_followed_lists() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuth::new(bearer_code);
    let builder =
        get_2_users_id_followed_lists::Api::all("1660518823991336966").build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_users_id_followed_lists::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);

    if let Some(next_token) = response.meta.unwrap().next_token {
        let builder = get_2_users_id_followed_lists::Api::all("1660518823991336966")
            .pagination_token(&next_token)
            .build(&bearer_auth);
        let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
        println!("\n{}", serde_json::to_string(&res).unwrap());
        let response = serde_json::from_value::<get_2_users_id_followed_lists::Response>(res)?;
        assert_eq!(response.is_empty_extra(), true);
    }

    Ok(())
}
