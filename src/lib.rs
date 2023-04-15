pub mod api;
pub mod rate_limit;
pub mod error;
pub mod fields;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::api;

    #[tokio::test]
    async fn it_works() {
        let _res = api::get_2_tweets_id_liking_users::Api::new("123")
            .max_results(10)
            .pagination_token("xxxx")
            .execute("xxx").await;
    }
}
