use twapi_v2::{
    api::get_2_tweets_search_stream,
};
use futures_util::StreamExt;

// BEARER_CODE=XXXXX cargo run

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let response = get_2_tweets_search_stream::Api::open(&bearer_code).build().send().await?;
    if !response.status().is_success() {
        let json = response.json().await?;
        println!("{:?}", json);
        panic!();
    }
    let mut stream = response.bytes_stream();
    let mut data = String::new();
    while let Some(item) = stream.next().await {
        let s = String::from_utf8((*item?).to_vec())?;
        if data.is_empty() {
            data = s
        } else {
            data.push_str(&s);
        }
        let json: serde_json::Value = match serde_json::from_str(&data) {
            Ok(json) => json,
            _ => continue,
        };
        data = String::new();
        println!("{}", serde_json::to_string(&json).unwrap());
        let response = serde_json::from_value::<get_2_tweets_search_stream::Response>(json)?;
        assert_eq!(response.is_empty_extra(), true);
    }
    Ok(())
}
