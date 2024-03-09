## Changes

### v0.12.0 (2024/03/09)
* Supported mocks. For example, mockito.

### v0.11.0 (2024/03/08)
* add media_source_tweet_id in attachments
* remove rate_limit
* add headers

### v0.10.1 (2024/01/23)
* modify trend response

### v0.10.0 (2024/01/22)
* add user filed connection_status and most_recent_tweet_id

### v0.9.0 (2023/12/16)
* add get /2/users/search
- add get /2/trend/by/woeid/:woeid

### v0.8.1 (2023/12/06)
* feature models

### v0.8.0 (2023/12/06)
* add v1 to v2 json parser
* skip_serializing_if Option::is_none
* add poll_ids in attatchments 
* add like_count in users public_metrics

### v0.7.1 (2023/09/05)
* modify calucurate oauth1.0a path paramter

### v0.7.0 (2023/09/05)
* breaking change oauth11a to oauth10a
* updated crates

### v0.6.0 (2023/08/28)
* breaking change separate authentication
* support OAuth1.1a authentication. feature oauth11a

### v0.5.10 (2023/08/01)
* modified typo
* modified test status
* add note_tweet in tweets
* add bookmark_count in public_metrics

### v0.5.9 (2023/05/11)
* add post_2_oauth2_token_refresh_token
* remove post_2_oauth2_token
* serde rename for request enum
* test get_2_compliance_jobs
* modify parameters

### v0.5.8 (2023/05/08)
* typo README
* remove no use files
* add test status
* modify parameters

### v0.5.7 (2023/05/04)
* test get_2_users_id_mention etc...
* modify parameters

### v0.5.6 (2023/05/03)
* add streaming example
* test post_2_tweets_search_stream_rules etc...
* modify post_2_auth2_token interface
* modify parameters

### v0.5.6 (2023/05/03)
* add streaming example
* test post_2_tweets_search_stream_rules etc...
* modify post_2_auth2_token interface
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