use anyhow::Result;
use twapi_v2::oauth::{TwitterOauth, TwitterScope};

// API_KEY_CODE=XXXXX API_SECRET_CODE=XXXXX cargo test test_client_credentials --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_client_credentials() -> Result<()> {
    let api_key_code = match std::env::var("API_KEY_CODE") {
        Ok(api_key_code) => api_key_code,
        _ => return Ok(()),
    };
    let api_secret_code = match std::env::var("API_SECRET_CODE") {
        Ok(api_secret_code) => api_secret_code,
        _ => return Ok(()),
    };

    let oauth = TwitterOauth::new(
        &api_key_code,
        &api_secret_code,
        "http://localhost:3000",
        TwitterScope::all(),
    )?;
    let res = oauth.client_credentials().await?;
    println!("{:?}", res);
    Ok(())
}
