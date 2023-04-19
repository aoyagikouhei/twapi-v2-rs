pub mod api;
pub mod error;
pub mod fields;
pub mod rate_limit;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use reqwest::StatusCode;

    use crate::{
        api::{
            execute_retry,
            get_2_tweets_id::{Api, Expansions},
        },
        fields::{
            media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
            tweet_fields::TweetFields, user_fields::UserFields,
        },
    };

    #[tokio::test]
    async fn it_works() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();

        let builder = Api::new(&bearer_code, "1432976528447442945")
            .expansions(Expansions::all())
            .tweet_fields(TweetFields::open())
            .user_fields(UserFields::all())
            .media_fields(MediaFields::all())
            .place_fields(PlaceFields::all())
            .poll_fields(PollFields::all())
            .build();

        let res = execute_retry(
            builder,
            2,
            &vec![StatusCode::UNAUTHORIZED],
            &|it| println!("{:?}", it),
            Some(Duration::from_secs(5)),
            None,
        )
        .await;
        println!("{:?}", res);
    }
}
