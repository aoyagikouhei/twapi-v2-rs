use std::time::Duration;

use reqwest::{RequestBuilder, StatusCode};
use tokio::time::{sleep, timeout};

use crate::{
    api::{execute_twitter, TwitterResult},
    error::Error,
};

pub trait RetryLogger {
    fn log(&self, builder: &RequestBuilder);
}

pub async fn execute_retry(
    builder: RequestBuilder,
    retry_count: usize,
    retryable_status_codes: &[StatusCode],
    retry_logger: Option<&impl RetryLogger>,
    timeout_duration: Option<Duration>,
    retry_delay_secound_count: Option<u64>,
) -> TwitterResult {
    let mut count: usize = 0;

    loop {
        let target = builder
            .try_clone()
            .ok_or(Error::Other("builder clone fail".to_owned(), None))?;
        if let Some(retry_logger) = retry_logger {
            retry_logger.log(&target);
        }

        let error = if let Some(timeout_duration) = timeout_duration {
            match timeout(timeout_duration, execute_twitter(target)).await {
                Ok(res) => match res {
                    Ok(res) => return Ok(res),
                    Err(err) => match &err {
                        Error::Twitter(twitter_error, _, _) => {
                            if !retryable_status_codes.contains(&twitter_error.status_code) {
                                return Err(err);
                            }
                            err
                        }
                        _ => return Err(err),
                    },
                },
                Err(_) => Error::Timeout,
            }
        } else {
            match execute_twitter(target).await {
                Ok(res) => return Ok(res),
                Err(err) => match &err {
                    Error::Twitter(twitter_error, _, _) => {
                        if !retryable_status_codes.contains(&twitter_error.status_code) {
                            return Err(err);
                        }
                        err
                    }
                    _ => return Err(err),
                },
            }
        };
        if count >= retry_count {
            return Err(error);
        }
        count += 1;
        sleep_sec(retry_delay_secound_count, count).await;
    }
}

async fn sleep_sec(retry_delay_secound_count: Option<u64>, count: usize) {
    let seconds = retry_delay_secound_count.unwrap_or(2_i64.pow(count as u32) as u64);
    sleep(Duration::from_secs(seconds)).await;
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use reqwest::{RequestBuilder, StatusCode};

    use crate::{
        api::get_2_tweets_id::{Api, Expansions, Response},
        fields::{
            media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
            tweet_fields::TweetFields, user_fields::UserFields,
        },
        retry::execute_retry,
    };

    use super::RetryLogger;

    struct Logger;
    impl RetryLogger for Logger {
        fn log(&self, builder: &RequestBuilder) {
            println!("{:?}", builder);
        }
    }

    // BEARER_CODE=XXXXX TWEET_ID=XXXX cargo test --features retry -- --nocapture

    #[tokio::test]
    async fn it_works() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let tweet_id = std::env::var("TWEET_ID").unwrap_or_default();
        let logger = Logger {};

        let builder = Api::new(&bearer_code, &tweet_id)
            .expansions(Expansions::all())
            .tweet_fields(TweetFields::open())
            .user_fields(UserFields::all())
            .media_fields(MediaFields::all())
            .place_fields(PlaceFields::all())
            .poll_fields(PollFields::all())
            .build();

        let res = execute_retry(
            builder,
            2,
            &vec![StatusCode::UNAUTHORIZED],
            Some(&logger),
            Some(Duration::from_secs(10)),
            None,
        )
        .await;
        match res {
            Ok(res) => {
                println!("{}", res.0.to_string());
                let val = serde_json::from_value::<Response>(res.0);
                println!("{:?}", val);
            }
            _ => {}
        }
    }
}
