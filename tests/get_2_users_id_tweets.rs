use anyhow::Result;
use twapi_v2::{
    api::{BearerAuthentication, execute_twitter, get_2_users_id_tweets},
    error::Error,
};

//BEARER_CODE=XXXXX cargo test --test get_2_users_id_tweets --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_id_tweets() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    match execute_twitter::<serde_json::Value>(
        || get_2_users_id_tweets::Api::all("19522946").build(&bearer_auth),
        &None,
    )
    .await
    {
        Ok((res, rate_limit)) => {
            println!("{}", serde_json::to_string(&res).unwrap());
            println!("{}", rate_limit);
            let response = serde_json::from_value::<get_2_users_id_tweets::Response>(res)?;
            assert_eq!(response.is_empty_extra(), true);
        }
        Err(Error::Twitter(e, _, header)) => {
            println!("Error: {:?}", e);
            println!("Hedder: {:?}", header);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}
