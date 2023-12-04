// cargo test --all-features test_models_from_v1 --  --nocapture --test-threads=1

use std::{fs::File, io::Write};

use twapi_v2::models::TweetModel;

#[tokio::test]
async fn test_models_from_v1() -> anyhow::Result<()> {
    use std::io::Read;
    let mut file = File::open("samples/1703799900679589991_v1.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let src = serde_json::from_str::<serde_json::Value>(&data)?;
    let res = TweetModel::from_v1(&src);
    let mut file = File::create("result.json")?;
    write!(file, "{}", serde_json::to_string_pretty(&res).unwrap())?;
    file.flush()?;
    Ok(())
}
