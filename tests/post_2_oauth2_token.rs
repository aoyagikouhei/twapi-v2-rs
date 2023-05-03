use anyhow::Result;
use twapi_v2::api::{execute_twitter, post_2_oauth2_token};

// API_KEY_CODE=XXXXX API_SECRET_CODE=XXXXX REFRESH_TOKEN=XXXXX cargo test test_post_2_oauth2_token -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_oauth2_token() -> Result<()> {
    let api_key_code = match std::env::var("API_KEY_CODE") {
        Ok(api_key_code) => api_key_code,
        _ => return Ok(()),
    };
    let api_secret_code = match std::env::var("API_SECRET_CODE") {
        Ok(api_secret_code) => api_secret_code,
        _ => return Ok(()),
    };
    let refresh_token = match std::env::var("REFRESH_TOKEN") {
        Ok(refresh_token) => refresh_token,
        _ => return Ok(()),
    };
    let builder = post_2_oauth2_token::Api::new(
        &api_key_code,
        &api_secret_code,
        &api_key_code,
        post_2_oauth2_token::GrantType::RefreshToken,
        &refresh_token,
    )
    .build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_oauth2_token::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
