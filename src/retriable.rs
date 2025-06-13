use std::time::Duration;

use reqwest::{RequestBuilder, Response, StatusCode};
use reqwest_builder_retry::RetryType;
use serde::de::DeserializeOwned;

use crate::{
    error::{Error, TwitterError},
    headers::Headers,
};

pub async fn execute_retry<T, F>(
    make_builder: F,
    try_count: u8,
    retry_duration: Duration,
) -> Result<RetryResult<T>, Error>
where
    T: DeserializeOwned,
    F: Fn() -> RequestBuilder,
{
    match reqwest_builder_retry::convenience::execute(
        |_| {
            let builder = make_builder();
            tracing::trace!(?builder, "request");
            builder
        },
        |response| {
            check_done::<T>(
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
pub struct RetryResult<T> {
    pub result: T,
    pub status_code: StatusCode,
    pub headers: Headers,
}

async fn check_done<T>(
    response: Result<Response, reqwest::Error>,
    retryable_status_codes: &[StatusCode],
) -> Result<RetryResult<T>, (RetryType, crate::error::Error)>
where
    T: DeserializeOwned,
{
    tracing::trace!(?response, "response");
    match response {
        Ok(response) => {
            let status_code = response.status();
            let headers = Headers::new(response.headers());
            let text = response.text().await.map_err(|err| {
                (
                    RetryType::Retry,
                    Error::Other(err.to_string(), Some(status_code)),
                )
            })?;
            let json = serde_json::from_str::<serde_json::Value>(&text).map_err(|err| {
                (
                    RetryType::Retry,
                    Error::Other(err.to_string(), Some(status_code)),
                )
            })?;
            if status_code.is_success() {
                if let Ok(result) = serde_json::from_value::<T>(json.clone()) {
                    return Ok(RetryResult {
                        result,
                        status_code,
                        headers,
                    });
                }
            }
            Err((
                calc_retry_type(status_code, retryable_status_codes),
                Error::Twitter(
                    TwitterError::new(&json, status_code),
                    json,
                    Box::new(headers),
                ),
            ))
        }
        Err(err) => Err((RetryType::Retry, Error::Other(err.to_string(), None))),
    }
}

pub(crate) fn calc_retry_type(status_code: StatusCode, retryable_status_codes: &[StatusCode]) -> RetryType {
    if retryable_status_codes.contains(&status_code) || status_code.is_success() {
        RetryType::Retry
    } else {
        RetryType::Stop
    }
}
