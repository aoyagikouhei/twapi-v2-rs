use twapi_v2::{
    api::get_2_tweets_search_stream as api
    //api::get_2_tweets_compliance_stream as api
    //api::get_2_tweets_sample_stream as api
    //api::get_2_tweets_sample10_stream as api
};
use futures_util::StreamExt;

// BEARER_CODE=XXXXX cargo run

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let response = api::Api::open(&bearer_code).build().send().await?;
    //let response = api::Api::new(&bearer_code, 1).build().send().await?;
    if !response.status().is_success() {
        println!("{:?}", response.status());
        let json: serde_json::Value = response.json().await?;
        println!("{:?}", json);
        panic!();
    }
    let mut stream = response.bytes_stream();
    let mut data = String::new();
    while let Some(item) = stream.next().await {
        let s = String::from_utf8((*item?).to_vec())?;
        println!("{}", s);
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
        let response = serde_json::from_value::<api::Response>(json)?;
        assert_eq!(response.is_empty_extra(), true);
    }
    Ok(())
}
