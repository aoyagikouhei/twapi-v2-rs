## Changes

### v0.26.0 (2026/02/14)
* Add retriable_fn
* Remove retriable_status_code

### v0.25.0 (2026/02/14)
* Remove retry and upload
* Change default support retry

### v0.24.1 (2026/02/13)
* Modify docs

### v0.24.0 (2026/02/13)
* Add PartialEq in Responses

### v0.23.0 (2026/01/30)
* Update Edtion 2024
* Remove oauth feature. Instead of use [twapi-oauth2](https://crates.io/crates/twapi-oauth2)
* Update reqwest 0.13

### v0.22.0 (2026/01/29)
* edit_history_tweet_ids to option

### v0.21.0 (2025/10/07)
* modify media upload by binary
- modify date rfc3339

### v0.20.0 (2025/05/02)
* modify media upload

### v0.19.1 (2025/03/31)
* add post_2_oauth2_token_refresh_token TwapiOptions

### v0.19.0 (2025/03/31)
* add oauth token TwapiOptions

### v0.18.2 (2025/02/17)
* modify post_2_media_upload_init MediaCategory

### v0.18.1 (2025/02/13)
* add retry fn

### v0.18.0 (2025/02/11)
* Update rand 0.9
* Add /2/media/metadata/create
* Add /2/media/subtitles/create
* Add /2/media/subtitles/delete

### v0.17.1 (2025/01/23)
* Modify get metadata async in upload_v2

### v0.17.0 (2025/01/23)
* Update Oauth2 v0.5.0
* Modify support file reading async in upload_v2

### v0.16.1 (2025/01/23)
* Add media.write in Scopes
* modify api.x.com from api.twitter.com

### v0.16.0 (2025/01/22)
* Add community_id in tweets
* Add max_trends in treand api
* Add v2 media upload(However, when I executed it, I got a 403 and was not successful.)
* Modify Tweet.fields, User.fields
* Update crates

### v0.15.2 (2024/11/28)
* Add media_count in users/public_metrics
* Update crates

### v0.15.1 (2024/06/21)
* Modify get_2_tweets_search_stream_rules response

### v0.15.0 (2024/05/25)
* Add timeout for api

### v0.14.2 (2024/04/04)
* Add check_processing for upload
* Add post-media example

### v0.14.1 (2024/04/02)
* Orverride prefix url for each APIs

### v0.14.0 (2024/03/27)
* Add upload.twitter.com APIs
* Add GET /2/usage/tweets

### v0.13.1 (2024/03/25)
* Modify oauth with state

### v0.13.0 (2024/03/25)
* Updated reqwest 0.12

### v0.12.0 (2024/03/09)
* Supported mocks. For example, mockito.

### v0.11.0 (2024/03/08)
* Add media_source_tweet_id in attachments
* Remove rate_limit
* Add headers

### v0.10.1 (2024/01/23)
* Modify trend response

### v0.10.0 (2024/01/22)
* Add user filed connection_status and most_recent_tweet_id

### v0.9.0 (2023/12/16)
* Add get /2/users/search
- Add get /2/trend/by/woeid/:woeid

### v0.8.1 (2023/12/06)
* Add feature models

### v0.8.0 (2023/12/06)
* Add v1 to v2 json parser
* skip_serializing_if Option::is_none
* Add poll_ids in attatchments 
* Add like_count in users public_metrics

### v0.7.1 (2023/09/05)
* Modify calucurate oauth1.0a path paramter

### v0.7.0 (2023/09/05)
* Breaking change oauth11a to oauth10a
* Updated crates

### v0.6.0 (2023/08/28)
* Breaking change separate authentication
* Support OAuth1.1a authentication. feature oauth11a

### v0.5.10 (2023/08/01)
* Modified typo
* Modified test status
* Add note_tweet in tweets
* Add bookmark_count in public_metrics

### v0.5.9 (2023/05/11)
* Add post_2_oauth2_token_refresh_token
* Remove post_2_oauth2_token
* Add serde rename for request enum
* Test get_2_compliance_jobs
* Modify parameters

### v0.5.8 (2023/05/08)
* Modify typo README
* Remove no use files
* Add test status
* Modify parameters

### v0.5.7 (2023/05/04)
* Test get_2_users_id_mention etc...
* Modify parameters

### v0.5.6 (2023/05/03)
* Add streaming example
* Test post_2_tweets_search_stream_rules etc...
* Modify post_2_auth2_token interface
* Modify parameters

### v0.5.6 (2023/05/03)
* Add streaming example
* Test post_2_tweets_search_stream_rules etc...
* Modify post_2_auth2_token interface
* Modify parameters

### v0.5.5 (2023/05/02)
* Test post_lists_id_members etc...
* Modify parameters

### v0.5.4 (2023/04/28)
* Test post_2_users_id_retweets, post_2_users_id_likes, post_2_users_id_muting, post_2_users_id_following, post_2_users_id_blocking, post_2_users_id_bookmarks, etc...
* Modify parameters

### v0.5.3 (2023/04/27)
* Test post_2_tweets, delete_2_tweets_id, post_2_dm_conversations_with_participant_id_message, etc...
* Modify parameters

### v0.5.2 (2023/04/26)
* Test get_2_users_me, get_2_tweets_search_recent
* Modify response parameters

### v0.5.1 (2023/04/25)
* Remove unuse url crate
* Add pub use reqwest

### v0.5.0 (2023/04/25)
* Add Api::all, Api::open methods. It's all enum parameter setted.
* In Api::all and Api::open methods, max_results is max value.

### v0.4.0 (2023/04/25)
* Twitter OAuth
* Add oauth-web example

### v0.3.0 (2023/04/24)
* Support api::execute_twitter generics parameter
* Api::execue method return specific type. (If you want to use serde_json::Value, use execute_twitter directly.)

### v0.2.0 (2023/04/23)
* Experimental type support.

### v0.1.0 (2023/04/20)
* First release.