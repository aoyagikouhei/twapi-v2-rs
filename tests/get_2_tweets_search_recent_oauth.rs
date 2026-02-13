use anyhow::Result;
use mockito::Server;
use serde_json::json;
use twapi_v2::api::{self, execute_twitter, get_2_tweets_search_recent};
use twapi_v2::error::Error;
use twapi_v2::oauth10a::OAuthAuthentication;

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX cargo test test_get_2_tweets_search_recent_oauth --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_search_recent_oauth() -> Result<()> {
    api::clear_prefix_url();
    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let (res, headers) = execute_twitter::<serde_json::Value>(
        || {
            get_2_tweets_search_recent::Api::open("東京")
                .max_results(10)
                .build(&auth)
        },
        &None,
    )
    .await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    println!("{}", headers);
    let response = serde_json::from_value::<get_2_tweets_search_recent::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX cargo test test_get_2_tweets_search_recent_oauth_mock --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_search_recent_oauth_mock() -> Result<()> {
    api::clear_prefix_url();
    let mut server = Server::new_async().await;
    api::setup_prefix_url(&server.url());

    let mock = server
        .mock("GET", "/2/tweets/search/recent")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{ \"origin\": \"0.0.0.0\" }")
        .create_async()
        .await;

    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );

    let twapi_options = api::TwapiOptions {
        prefix_url: Some(server.url()),
        ..Default::default()
    };

    let (res, _headers) = execute_twitter::<get_2_tweets_search_recent::Response>(
        || {
            get_2_tweets_search_recent::Api::open("東京")
                .max_results(10)
                .twapi_options(twapi_options.clone())
                .build(&auth)
        },
        &None,
    )
    .await?;
    assert_eq!(res.extra.get("origin"), Some(&json!("0.0.0.0")));
    mock.assert();
    Ok(())
}

// CONSUMER_KEY=XXXX CONSUMER_SECRET=XXXX ACCESS_KEY=XXXX ACCESS_SECRET=XXXX cargo test test_get_2_tweets_search_recent_oauth_mock_rate_limet --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_search_recent_oauth_mock_rate_limet() -> Result<()> {
    let mut server = Server::new_async().await;
    api::setup_prefix_url(&server.url());

    let data = json!({
        "status": "error",
        "title": "Too Many Requests",
    });

    let mock = server
        .mock("GET", "/2/tweets/search/recent")
        .match_query(mockito::Matcher::Any)
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_body(data.to_string())
        .create_async()
        .await;

    let auth = OAuthAuthentication::new(
        std::env::var("CONSUMER_KEY").unwrap_or_default(),
        std::env::var("CONSUMER_SECRET").unwrap_or_default(),
        std::env::var("ACCESS_KEY").unwrap_or_default(),
        std::env::var("ACCESS_SECRET").unwrap_or_default(),
    );
    let res = execute_twitter::<get_2_tweets_search_recent::Response>(
        || {
            get_2_tweets_search_recent::Api::open("東京")
                .max_results(10)
                .build(&auth)
        },
        &None,
    )
    .await;
    match res {
        Err(Error::Twitter(e, _value, _headers)) => {
            assert_eq!(e.status_code.as_u16(), 429);
            assert_eq!(e.title, "Too Many Requests");
        }
        _ => panic!("unexpected error"),
    }
    mock.assert();
    Ok(())
}
