use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_compliance_jobs_id};

// BEARER_CODE=XXXXX JOB_ID=XXXXX cargo test test_get_2_compliance_jobs_id -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_compliance_jobs_id() -> Result<()> {
    let job_id = match std::env::var("JOB_ID") {
        Ok(job_id) => job_id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = get_2_compliance_jobs_id::Api::new(&bearer_code, &job_id).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_compliance_jobs_id::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
