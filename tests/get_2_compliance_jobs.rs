use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_compliance_jobs};

// BEARER_CODE=XXXXX cargo test test_get_2_compliance_jobs -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_compliance_jobs() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder =
        get_2_compliance_jobs::Api::new(&bearer_code, get_2_compliance_jobs::Type::Users).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_compliance_jobs::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
