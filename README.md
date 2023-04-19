# twapi-v2-rs

Twitter v2 library.

[Documentation](https://docs.rs/twapi-v2)

## Features
- request builder
- retriable

## Changes

### v0.1.0 (2023/04/19)
* first release

## Example
```rust
use twapi_v2::{
    api::{get_2_tweets_id::{Api, Expansions}, execute_retry},
    fields::{
        media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
        tweet_fields::TweetFields, user_fields::UserFields,
    },
};

#[tokio::main]
async fn main() {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();

    let builder = Api::new(&bearer_code, "1432976528447442945")
        .expansions(Expansions::all())
        .tweet_fields(TweetFields::open())
        .user_fields(UserFields::all())
        .media_fields(MediaFields::all())
        .place_fields(PlaceFields::all())
        .poll_fields(PollFields::all())
        .build();

    let res = execute_retry(builder, 2, None, &vec![401], &|it| println!("{:?}", it)).await;
    println!("{:?}", res);
}
```