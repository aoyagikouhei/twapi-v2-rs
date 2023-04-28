# twapi-v2-rs

Twitter API v2 library.

[Documentation](https://docs.rs/twapi-v2)

- Request builder
- Retrive rate limit from response headers
- Convenience setted parameter methods
- Optional retriable and timeout and logging
- Optional OAuth with web sample
- **Experimental** type support.

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
[CHANGELOG.md](https://github.com/aoyagikouhei/twapi-v2-rs/blob/main/CHANGELOG.md)

## Examples

### API
```rust
use twapi_v2::api::get_2_tweets_id;

#[tokio::main]
async fn main() {
    let bearer_code = std::env::var("BEARER_CODE").unwrap();
    let tweet_id = std::env::var("TWEET_ID").unwrap();
    let res = get_2_tweets_id::Api::open(&bearer_code, &tweet_id)
        .execute()
        .await;
    if let Some((val, rate_limit)) = res {
        println!("{:?}", val);
    }
}
```

### Twitter OAuth Web
```
cd examples/oauth-web
API_KEY_CODE=XXXX API_SECRET_CODE=XXXX cargo run
```
http://localhost:3000/
