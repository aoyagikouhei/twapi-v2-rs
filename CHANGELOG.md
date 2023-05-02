## Changes

### v0.5.6 (2023/05/03)
* add streaming example
* test post_2_tweets_search_stream_rules etc...
* modify parameters

### v0.5.5 (2023/05/02)
* test post_lists_id_members etc...
* modify parameters

### v0.5.4 (2023/04/28)
* test post_2_users_id_retweets, post_2_users_id_likes, post_2_users_id_muting, post_2_users_id_following, post_2_users_id_blocking, post_2_users_id_bookmarks, etc...
* modify parameters

### v0.5.3 (2023/04/27)
* test post_2_tweets, delete_2_tweets_id, post_2_dm_conversations_with_participant_id_message, etc...
* modify parameters

### v0.5.2 (2023/04/26)
* test get_2_users_me, get_2_tweets_search_recent
* modify response parameters

### v0.5.1 (2023/04/25)
* remove unuse url crate
* pub use reqwest

### v0.5.0 (2023/04/25)
* Add Api::all, Api::open methods. It's all enum parameter setted.
* In Api::all and Api::open methods, max_results is max value.

### v0.4.0 (2023/04/25)
* Twitter OAuth
* oauth-web example

### v0.3.0 (2023/04/24)
* Support api::execute_twitter generics parameter
* Api::execue method return specific type. (If you want to use serde_json::Value, use execute_twitter directly.)

### v0.2.0 (2023/04/23)
* Experimental type support.

### v0.1.0 (2023/04/20)
* First release.