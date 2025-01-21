use std::{
    io::{BufReader, Cursor, Read},
    path::PathBuf,
};

use crate::{
    api::{
        get_2_media_upload::{self, Command},
        post_2_media_upload_append, post_2_media_upload_finalize,
        post_2_media_upload_init::{self, MediaCategory},
        Authentication, TwapiOptions,
    },
    error::Error,
    headers::Headers,
    responses::processing_info::{ProcessingInfo, State},
};

fn get_file_size(path: &PathBuf) -> Result<u64, Error> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

pub async fn upload_media(
    path: &PathBuf,
    media_type: &str,
    media_category: Option<MediaCategory>,
    additional_owners: Option<String>,
    authentication: &impl Authentication,
    twapi_options: Option<&TwapiOptions>,
) -> Result<(post_2_media_upload_finalize::Response, Headers), Error> {
    // INIT
    let file_size = get_file_size(path)?;
    let media_id = execute_init(
        file_size,
        media_type,
        media_category,
        additional_owners,
        authentication,
        twapi_options,
    )
    .await?;
    tracing::info!(media_id = media_id, "post_media_upload_init");

    // APPEND
    execute_append(path, authentication, file_size, &media_id, twapi_options).await?;

    // FINALIZE
    let data = post_2_media_upload_finalize::FormData {
        media_id: media_id.clone(),
    };
    let mut api = post_2_media_upload_finalize::Api::new(data);
    if let Some(twapi_options) = twapi_options {
        api = api.twapi_options(twapi_options.clone());
    }
    let res = api.execute(authentication).await;
    tracing::info!(media_id = media_id, "post_media_upload_finalize");
    res
}

async fn execute_init(
    file_size: u64,
    media_type: &str,
    media_category: Option<MediaCategory>,
    additional_owners: Option<String>,
    authentication: &impl Authentication,
    twapi_options: Option<&TwapiOptions>,
) -> Result<String, Error> {
    let data = post_2_media_upload_init::FormData {
        total_bytes: file_size,
        media_type: media_type.to_owned(),
        media_category,
        additional_owners,
    };
    let mut api = post_2_media_upload_init::Api::new(data);
    if let Some(twapi_options) = twapi_options {
        api = api.twapi_options(twapi_options.clone());
    }
    let (response, _) = api.execute(authentication).await?;
    let media_id = response.data.and_then(|it| it.id).unwrap_or_default();
    Ok(media_id)
}

pub fn get_media_id(response: &post_2_media_upload_finalize::Response) -> String {
    let Some(data) = &response.data else {
        return "".to_owned();
    };
    data.id.clone().unwrap_or_default()
}

async fn execute_append(
    path: &PathBuf,
    authentication: &impl Authentication,
    file_size: u64,
    media_id: &str,
    twapi_options: Option<&TwapiOptions>,
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
        let data = post_2_media_upload_append::FormData {
            media_id: media_id.to_owned(),
            segment_index,
            cursor,
        };
        let mut api = post_2_media_upload_append::Api::new(data);
        if let Some(twapi_options) = twapi_options {
            api = api.twapi_options(twapi_options.clone());
        }
        let _ = api.execute(authentication).await?;
        tracing::info!(
            segment_index = segment_index,
            media_id = media_id,
            "post_media_upload_append"
        );
        segment_index += 1;
    }
    Ok(())
}

fn get_check_after_secs(processing_info: &Option<ProcessingInfo>) -> Option<i64> {
    let Some(ref processing_info) = processing_info else {
        return None;
    };
    let state = &(processing_info.state.clone()?);
    match state {
        State::Pending | State::InProgress => processing_info.check_after_secs,
        _ => None,
    }
}

fn calc_progress_percent(res: &get_2_media_upload::Response) -> i64 {
    let Some(data) = &res.data else {
        return 0;
    };
    let Some(ref processing_info) = data.processing_info else {
        return 0;
    };
    processing_info.progress_percent.unwrap_or(0)
}

pub async fn check_processing(
    response: post_2_media_upload_finalize::Response,
    authentication: &impl Authentication,
    f: Option<impl Fn(i64, &get_2_media_upload::Response, &Headers) -> Result<(), Error>>,
    twapi_options: Option<&TwapiOptions>,
) -> Result<(), Error> {
    let Some(data) = response.data else {
        return Err(Error::Other("data not found".to_owned(), None));
    };
    let Some(media_id) = data.id else {
        return Err(Error::Other("media_id not found".to_owned(), None));
    };
    let mut processing_info = data.processing_info;
    let mut count = 0;
    loop {
        let Some(check_after_secs) = get_check_after_secs(&processing_info) else {
            return Ok(());
        };
        tokio::time::sleep(std::time::Duration::from_secs(check_after_secs as u64)).await;
        let mut api = get_2_media_upload::Api::new(&media_id, Command::Status);
        if let Some(twapi_options) = twapi_options {
            api = api.twapi_options(twapi_options.clone());
        }
        let (res, header) = api.execute(authentication).await?;
        tracing::info!(
            count = count,
            media_id = media_id,
            progress_percent = calc_progress_percent(&res),
            "get_media_upload"
        );
        if let Some(ref f) = f {
            f(count, &res, &header)?;
        }
        processing_info = res.data.and_then(|it| it.processing_info);
        count += 1;
    }
}
