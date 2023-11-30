// cargo test --all-features test_models_from_v1 --  --nocapture --test-threads=1

use twapi_v2::models::TweetModel;

#[tokio::test]
async fn test_models_from_v1() -> anyhow::Result<()> {
    use std::io::Read;
    let mut file = std::fs::File::open("v1.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let src = serde_json::from_str::<serde_json::Value>(&data)?;
    println!("{:#?}", TweetModel::from_v1(&src));
    Ok(())
}
