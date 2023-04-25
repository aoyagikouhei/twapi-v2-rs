use anyhow::Result;
use twapi_v2::api::get_2_users_me;

// BEARER_CODE=XXXXX cargo test test_get_2_users_me -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_me() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let (res, _rate_limit) = get_2_users_me::Api::all(&bearer_code).execute().await?;
    println!("{:?}", res);
    assert_eq!(res.data.unwrap().username, "aoyagikouhei");
    Ok(())
}
