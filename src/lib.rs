pub mod api;
pub mod error;
pub mod rate_limit;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::api;

    #[tokio::test]
    async fn it_works() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let mut tweet_fields = api::get_2_tweets_id::TweetFields::all();
        tweet_fields.remove(&api::get_2_tweets_id::TweetFields::NonPublicMetrics);
        tweet_fields.remove(&api::get_2_tweets_id::TweetFields::OrganicMetrics);
        tweet_fields.remove(&api::get_2_tweets_id::TweetFields::PromotedMetrics);

        let res = api::get_2_tweets_id::Api::new(&bearer_code, "1633027859752288256")
            .expansions(api::get_2_tweets_id::Expansions::all())
            .tweet_fields(tweet_fields)
            .user_fields(api::get_2_tweets_id::UserFields::all())
            .media_fields(api::get_2_tweets_id::MediaFields::all())
            .place_fields(api::get_2_tweets_id::PlaceFields::all())
            .poll_fields(api::get_2_tweets_id::PollFields::all())
            .execute()
            .await;
        println!("{:?}", res);
    }
}
