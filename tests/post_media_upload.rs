use std::path::PathBuf;

use anyhow::Result;
use twapi_v2::{
    oauth10a::OAuthAuthentication,
    upload::{self, media_category::MediaCategory},
};

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX cargo test test_post_media_upload -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_media_upload() -> Result<()> {
    upload::clear_prefix_url();
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );

    let response = upload::upload_media(
        &PathBuf::from("plpgsql.jpg"),
        "image/jpeg",
        Some(MediaCategory::TweetImage),
        None,
        &auth,
    )
    .await?;
    println!("{:?}", response);
    Ok(())
}
