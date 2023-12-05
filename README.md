# twapi-v2-rs

Twitter API v2 library.

[Documentation](https://docs.rs/twapi-v2)

- Request builder
- Retrive rate limit from response headers
- Convenience setted parameter methods
- Bearer authentication(OAuth 2.0 Authorization Code Flow with PKCE)
- OAuth1.0a authentication(OAuth 1.0a User Contex)
- Optional retriable and timeout and logging
- Optional OAuth with web example
- Optional v1 to v2 parser
- Streaming example
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

### oauth10a
- Use api by OAuth1.0a

### models
- From v1 to v2

## Changes
[CHANGELOG.md](https://github.com/aoyagikouhei/twapi-v2-rs/blob/main/CHANGELOG.md)

## Test status
[TEST.md](https://github.com/aoyagikouhei/twapi-v2-rs/blob/main/TEST.md)

## Examples

### API(bearer)
```rust
use twapi_v2::api::{get_2_tweets_id, BearerAuthentication};

#[tokio::main]
async fn main() {
    let bearer_code = std::env::var("BEARER_CODE").unwrap();
    let auth = BearerAuthentication::new(bearer_code);
    let tweet_id = std::env::var("TWEET_ID").unwrap();
    let res = get_2_tweets_id::Api::open(&tweet_id)
        .execute(&auth)
        .await;
    if let Some((val, rate_limit)) = res {
        println!("{:?}", val);
    }
}
```

### API(OAuth1.0a)
```rust
use twapi_v2::api::{get_2_tweets_id, BearerAuthentication};
use twapi_v2::oauth10a::OAuthAuthentication;

#[tokio::main]
async fn main() {
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let tweet_id = std::env::var("TWEET_ID").unwrap();
    let res = get_2_tweets_id::Api::open(&tweet_id)
        .execute(&auth)
        .await;
    if let Some((val, rate_limit)) = res {
        println!("{:?}", val);
    }
}
```

### Twitter OAuth Web
```
cd examples/oauth-web
API_KEY_CODE=XXXX API_SECRET_CODE=XXXX CALLBACK_URL=http://localhost:3000/oauth cargo run
```
http://localhost:3000/

### Streaming
```
cd examples/streaming
BEARER_CODE=XXXXX cargo run
```