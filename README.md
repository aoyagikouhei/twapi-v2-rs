# twapi-v2-rs

Twitter v2 library.

[Documentation](https://docs.rs/twapi-v2)

- Request builder
- Retrive rate limit from response headers.
- Default response type is serde_json::Value.
- Experimental type support.

## Features
### default
- reqwest/default-tls

### rustls-tls
- reqwest/rustls-tls

### retry
- Retriable
- Timeout
- Logging

### oauth
- Twitter OAuth

## Changes

### v0.4.0 (2023/04/25)
* Twitter OAuth

### v0.3.0 (2023/04/24)
* Support api::execute_twitter generics parameter
* Api::execue method return specific type. (If you want to use serde_json::Value, use execute_twitter directly.)

### v0.2.0 (2023/04/23)
* Experimental type support.

### v0.1.0 (2023/04/20)
* First release.

## Example
```rust
use twapi_v2::{
    api::{get_2_tweets_id::{Api, Expansions}, execute_twitter},
    fields::{
        media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
        tweet_fields::TweetFields, user_fields::UserFields,
    },
};

#[tokio::main]
async fn main() {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let tweet_id = std::env::var("TWEET_ID").unwrap_or_default();

    let res = Api::new(&bearer_code, &tweet_id)
        .expansions(Expansions::all())
        .tweet_fields(TweetFields::open())
        .user_fields(UserFields::all())
        .media_fields(MediaFields::all())
        .place_fields(PlaceFields::all())
        .poll_fields(PollFields::all())
        .build()
        .execute()
        .await;
    if let Some((val, rate_limit)) = res {
        println!("{:?}", res);
    }
}
```