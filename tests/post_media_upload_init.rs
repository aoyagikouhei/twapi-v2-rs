use anyhow::Result;
use twapi_v2::{
    api::execute_twitter,
    oauth10a::OAuthAuthentication,
    upload::{self, post_media_uploda_init, MediaCategory},
};

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX cargo test test_post_media_upload_init -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_media_upload_init() -> Result<()> {
    upload::clear_prefix_url();
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let data = post_media_uploda_init::Data {
        total_bytes: 104_101,
        media_type: "image/jpeg".to_string(),
        media_category: Some(MediaCategory::TweetImage),
        ..Default::default()
    };
    let builder = post_media_uploda_init::Api::new(data).build(&auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<upload::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
