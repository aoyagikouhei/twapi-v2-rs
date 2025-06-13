use std::time::Duration;

use reqwest::{RequestBuilder, Response, StatusCode};
use reqwest_builder_retry::RetryType;

use crate::{
    error::{Error, TwitterError},
    headers::Headers, retriable::calc_retry_type,
};

pub async fn execute_retry<F>(
    make_builder: F,
    try_count: u8,
    retry_duration: Duration,
) -> Result<RetryResult, Error>
where
    F: Fn() -> RequestBuilder,
{
    match reqwest_builder_retry::convenience::execute(
        |_| {
            let builder = make_builder();
            tracing::trace!(?builder, "request");
            builder
        },
        |response| {
            check_done(
                response,
                &[StatusCode::TOO_MANY_REQUESTS, StatusCode::FORBIDDEN],
            )
        },
        try_count,
        retry_duration,
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(retry_type) => match retry_type {
            reqwest_builder_retry::error::Error::NoTry => {
                Err(crate::error::Error::Other("no_try".to_string(), None))
            }
            reqwest_builder_retry::error::Error::TryOver(res) => Err(res),
            reqwest_builder_retry::error::Error::Stop(res) => Err(res),
        },
    }
}

#[derive(Debug, Clone)]
pub struct RetryResult {
    pub status_code: StatusCode,
    pub headers: Headers,
}

async fn check_done(
    response: Result<Response, reqwest::Error>,
    retryable_status_codes: &[StatusCode],
) -> Result<RetryResult, (RetryType, crate::error::Error)> {
    tracing::trace!(?response, "response");
    match response {
        Ok(response) => {
            let status_code = response.status();
            let headers = Headers::new(response.headers());
            if status_code.is_success() {
                return Ok(RetryResult {
                    status_code,
                    headers,
                });
            }
            // エラーの時はbodyの回収を試みる
            let text = response.text().await.ok();
            let json = match text {
                Some(text) => serde_json::from_str::<serde_json::Value>(&text).ok(),
                None => None,
            };
            let retry_type = calc_retry_type(status_code, retryable_status_codes);
            match json {
                Some(json) => Err((
                    retry_type,
                    Error::Twitter(
                        TwitterError::new(&json, status_code),
                        json,
                        Box::new(headers),
                    ),
                )),
                None => Err((
                    retry_type,
                    Error::Other(
                        "Failed to parse response body".to_string(),
                        Some(status_code),
                    ),
                )),
            }
        }
        Err(err) => Err((RetryType::Retry, Error::Other(err.to_string(), None))),
    }
}
