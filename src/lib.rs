pub mod api;
pub mod error;
pub mod fields;
pub mod rate_limit;

#[cfg(test)]
mod tests {
    use crate::{
        api::{self, execute_retry},
        fields::{
            media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
            tweet_fields::TweetFields, user_fields::UserFields,
        },
    };

    #[tokio::test]
    async fn it_works() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();

        let builder = api::get_2_tweets_id::Api::new(&bearer_code, "1633027859752288256")
            .expansions(api::get_2_tweets_id::Expansions::all())
            .tweet_fields(TweetFields::open())
            .user_fields(UserFields::all())
            .media_fields(MediaFields::all())
            .place_fields(PlaceFields::all())
            .poll_fields(PollFields::all())
            .build();

        let res = execute_retry(builder, 2, None, &vec![401], &|it| println!("{:?}", it)).await;
        println!("{:?}", res);
    }
}
