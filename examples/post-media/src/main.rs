use tracing::Level;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, Registry};
use twapi_v2::{
    api::{post_2_media_upload_init::MediaCategory, post_2_tweets::{self, Media}, BearerAuthentication},
    error::Error,
    upload_v2::{check_processing, get_media_id, upload_media},
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
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);

    //let res  = get_2_users_me::Api::new().execute(&bearer_auth).await?;
    //println!("{:?}", res);

    let (response, _header) = upload_media(
        &std::path::PathBuf::from("test.mp4"),
        "video/mp4",
        Some(MediaCategory::AmplifyVideo),
        None,
        &bearer_auth,
        None,
    )
    .await?;
    tracing::info!(response =? response, "upload_media");
    let media_id = get_media_id(&response);
    check_processing(
        response,
        &bearer_auth,
        Some(|count, _response: &_, _header: &_| {
            if count > 100 {
                Err(Error::Upload("over counst".to_owned()))
            } else {
                Ok(())
            }
        }),
        None,
    )
    .await?;
    let body = post_2_tweets::Body {
        text: Some(
            "It's media v2 test using twapi-v2 Rust library with file async. https://crates.io/crates/twapi-v2"
                .to_string(),
        ),
        media: Some(Media {
            media_ids: vec![media_id],
            ..Default::default()
        }),
        ..Default::default()
    };
    let (response, _header) = post_2_tweets::Api::new(body).execute(&bearer_auth).await?;
    tracing::info!(response =? response, "post_2_tweets");
    Ok(())
}
