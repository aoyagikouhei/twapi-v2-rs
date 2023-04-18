pub mod api;
pub mod error;
pub mod fields;
pub mod rate_limit;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{api, fields};

    #[tokio::test]
    async fn it_works() {
        let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
        let mut tweet_fields = fields::tweet_fields::TweetFields::all();
        tweet_fields.remove(&fields::tweet_fields::TweetFields::NonPublicMetrics);
        tweet_fields.remove(&fields::tweet_fields::TweetFields::OrganicMetrics);
        tweet_fields.remove(&fields::tweet_fields::TweetFields::PromotedMetrics);

        let res = api::get_2_tweets_id::Api::new(&bearer_code, "1633027859752288256")
            .expansions(api::get_2_tweets_id::Expansions::all())
            .tweet_fields(tweet_fields)
            .user_fields(fields::user_fields::UserFields::all())
            .media_fields(fields::media_fields::MediaFields::all())
            .place_fields(fields::place_fields::PlaceFields::all())
            .poll_fields(fields::poll_fields::PollFields::all())
            .execute()
            .await;
        println!("{:?}", res);
    }
}
