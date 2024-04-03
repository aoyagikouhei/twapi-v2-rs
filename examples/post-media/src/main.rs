use tracing::Level;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, Registry};
use twapi_v2::{
    api::post_2_tweets::{self, Media}, error::Error, oauth10a::OAuthAuthentication, upload::{self, check_processing, media_category::MediaCategory}
};

pub fn setup_tracing(name: &str) {
    let formatting_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);
    let filter = Targets::new()
        .with_target(name, Level::TRACE)
        .with_target("twapi_v2", Level::TRACE);

    let subscriber = Registry::default()
        .with(filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing("post_media");
    tracing::info!("start");
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let (response, _header) = upload::upload_media(
        &std::path::PathBuf::from("test.mp4"),
        "video/mp4",
        Some(MediaCategory::AmplifyVideo),
        None,
        &auth,
    )
    .await?;
    tracing::info!(response =? response, "upload_media");
    let media_id = response.media_id_string.clone();
    check_processing(response, &auth, Some(|count, _response: &_, _header: &_| {
        if count > 100 {
            Err(Error::Upload("over counst".to_owned()))
        } else {
            Ok(())
        }
    }))
    .await?;
    let body = post_2_tweets::Body {
        text: Some("It's media test using twapi-v2 Rust library.".to_string()),
        media: Some(Media {
            media_ids: vec![media_id],
            ..Default::default()
        }),
        ..Default::default()
    };
    let (response, _header) = post_2_tweets::Api::new(body).execute(&auth).await?;
    tracing::info!(response =? response, "post_2_tweets");
    Ok(())
}
