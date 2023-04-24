# twapi-v2-rs

Twitter v2 library.

[Documentation](https://docs.rs/twapi-v2)

- Request builder
- Retrive rate limit from response headers
- Convenience setted parameter methods
- Optional retriable and timeout and logging
- Optional OAuth with web sample
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

### v0.5.0 (2023/04/26)
* Add Api::all, Api::open methods. It's all enum parameter setted.
* In Api::all and Api::open methods, max_results is max value.

### v0.4.0 (2023/04/25)
* Twitter OAuth
* oauth-web example

### v0.3.0 (2023/04/24)
* Support api::execute_twitter generics parameter
* Api::execue method return specific type. (If you want to use serde_json::Value, use execute_twitter directly.)

### v0.2.0 (2023/04/23)
* Experimental type support.

### v0.1.0 (2023/04/20)
* First release.

## Examples

### API
```rust
use twapi_v2::api::get_2_tweets_id;

#[tokio::main]
async fn main() {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let tweet_id = std::env::var("TWEET_ID").unwrap_or_default();
    let res = get_2_tweets_id::Api::open(&bearer_code, &tweet_id)
        .execute()
        .await;
    if let Some((val, rate_limit)) = res {
        println!("{:?}", res);
    }
}
```

### Twitter OAuth Web
```
cd examples/oauth-web
API_KEY_CODE=XXXX API_SECRET_CODE=XXXX cargo run
```
http://localhost:3000/
