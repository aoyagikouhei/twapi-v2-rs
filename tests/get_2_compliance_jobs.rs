use anyhow::Result;
use twapi_v2::api::{BearerAuthentication, execute_twitter, get_2_compliance_jobs};

// APP_BEARER_CODE=XXXXX cargo test test_get_2_compliance_jobs -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_compliance_jobs() -> Result<()> {
    let app_bearer_code = match std::env::var("APP_BEARER_CODE") {
        Ok(app_bearer_code) => app_bearer_code,
        _ => return Ok(()),
    };
    let bearer_auth = BearerAuthentication::new(app_bearer_code);    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(|| get_2_compliance_jobs::Api::new(get_2_compliance_jobs::Type::Users).build(&bearer_auth), &None).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_compliance_jobs::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
