pub mod api;
pub mod error;
pub mod fields;
pub mod rate_limit;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::api;

    #[tokio::test]
    async fn it_works() {
        let _res = api::get_2_tweets_id_liking_users::Api::new("bearer_code", "id")
            .max_results(10)
            .pagination_token("xxxx")
            .execute()
            .await;
    }
}
