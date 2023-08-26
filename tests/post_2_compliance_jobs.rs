use anyhow::Result;
use twapi_v2::api::{execute_twitter, post_2_compliance_jobs, BearerAuthentication};

// BEARER_CODE=XXXXX JOB_NAME=XXXXX cargo test test_post_2_compliance_jobs -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_compliance_jobs() -> Result<()> {
    let job_name = match std::env::var("JOB_NAME") {
        Ok(job_name) => job_name,
        _ => return Ok(()),
    };
    let body = post_2_compliance_jobs::Body {
        r#type: post_2_compliance_jobs::Type::Users,
        name: Some(job_name),
        ..Default::default()
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let builder = post_2_compliance_jobs::Api::new(body).build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_compliance_jobs::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
