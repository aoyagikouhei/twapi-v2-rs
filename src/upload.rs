use std::{
    io::{BufReader, Cursor, Read},
    path::PathBuf,
};

use reqwest::RequestBuilder;

use crate::{
    api::{Authentication, TwapiOptions, make_url_with_prefix},
    error::Error,
    headers::Headers,
};

use self::{media_category::MediaCategory, response::Response};

pub mod get_media_upload;
pub mod media_category;
pub mod post_media_metadata_create;
pub mod post_media_subtitles_create;
pub mod post_media_subtitles_delete;
pub mod post_media_upload_append;
pub mod post_media_upload_finalize;
pub mod post_media_upload_init;
pub mod response;

const POSTFIX_URL: &str = "/1.1/media/upload.json";
const ENV_KEY: &str = "TWAPI_V2_MEDIA_API_PREFIX_API";
const PREFIX_URL_MEDIA: &str = "https://upload.twitter.com";

pub fn clear_prefix_url() {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { std::env::set_var(ENV_KEY, PREFIX_URL_MEDIA) };
}

pub fn setup_prefix_url(url: &str) {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { std::env::set_var(ENV_KEY, url) };
}

pub(crate) fn make_url(twapi_options: &Option<TwapiOptions>, postfix_url: Option<&str>) -> String {
    make_url_with_prefix(
        &std::env::var(ENV_KEY).unwrap_or(PREFIX_URL_MEDIA.to_owned()),
        twapi_options,
        postfix_url.unwrap_or(POSTFIX_URL),
    )
}

pub(crate) async fn execute_no_response(builder: RequestBuilder) -> Result<Headers, Error> {
    let response = builder.send().await?;
    let status_code = response.status();
    let header = response.headers();
    let headers = Headers::new(header);
    if status_code.is_success() {
        Ok(headers)
    } else {
        let body = response.text().await?;
        Err(Error::Other(body, Some(status_code)))
    }
}

pub async fn upload_media(
    path: &PathBuf,
    media_type: &str,
    media_category: Option<MediaCategory>,
    additional_owners: Option<String>,
    authentication: &impl Authentication,
) -> Result<(response::Response, Headers), Error> {
    // INIT
    let metadata = std::fs::metadata(path)?;
    let file_size = metadata.len();
    let data = post_media_upload_init::Data {
        total_bytes: file_size,
        media_type: media_type.to_owned(),
        media_category,
        additional_owners,
    };
    let (response, _) = post_media_upload_init::Api::new(data)
        .execute(authentication)
        .await?;
    let media_id = response.media_id_string;
    tracing::info!(media_id = media_id, "post_media_upload_init");

    // APPEND
    execute_append(path, authentication, file_size, &media_id).await?;

    // FINALIZE
    let data = post_media_upload_finalize::Data {
        media_id: media_id.clone(),
    };
    let res = post_media_upload_finalize::Api::new(data)
        .execute(authentication)
        .await;
    tracing::info!(media_id = media_id, "post_media_upload_finalize");
    res
}

async fn execute_append(
    path: &PathBuf,
    authentication: &impl Authentication,
    file_size: u64,
    media_id: &str,
) -> Result<(), Error> {
    let mut segment_index = 0;
    let f = std::fs::File::open(path)?;
    let mut reader = BufReader::new(f);
    while segment_index * 5000000 < file_size {
        let read_size: usize = if (segment_index + 1) * 5000000 < file_size {
            5000000
        } else {
            (file_size - segment_index * 5000000) as usize
        };
        let mut cursor = Cursor::new(vec![0; read_size]);
        reader.read_exact(cursor.get_mut())?;
        let data = post_media_upload_append::Data {
            media_id: media_id.to_owned(),
            segment_index,
            cursor,
        };
        let _ = post_media_upload_append::Api::new(data)
            .execute(authentication)
            .await?;
        tracing::info!(
            segment_index = segment_index,
            media_id = media_id,
            "post_media_upload_append"
        );
        segment_index += 1;
    }
    Ok(())
}

pub async fn check_processing(
    response: Response,
    authentication: &impl Authentication,
    f: Option<impl Fn(i64, &Response, &Headers) -> Result<(), Error>>,
) -> Result<(), Error> {
    let media_id = response.media_id_string.clone();
    let mut processing_info = response.processing_info;
    let mut count = 0;
    loop {
        let Some(ref info) = processing_info else {
            return Ok(());
        };
        if !info.state.is_continue() {
            return Ok(());
        }

        if let Some(check_after_secs) = info.check_after_secs {
            tokio::time::sleep(std::time::Duration::from_secs(check_after_secs)).await;
            let (res, header) = get_media_upload::Api::new(media_id.clone())
                .execute(authentication)
                .await?;
            let progress_percent = res
                .processing_info
                .as_ref()
                .map(|it| it.progress_percent.unwrap_or(0))
                .unwrap_or(0);
            tracing::info!(
                count = count,
                media_id = media_id,
                progress_percent = progress_percent,
                "get_media_upload"
            );
            if let Some(ref f) = f {
                f(count, &res, &header)?;
            }
            processing_info = res.processing_info;
        } else {
            return Err(Error::Upload("check_ofter_secs not found".to_owned()));
        }
        count += 1;
    }
}
