use std::collections::HashMap;

use chrono::prelude::*;

pub type TweetModel = crate::api::get_2_tweets_id::Response;

fn str_to_utc(src: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_str(src, "%a %b %d %H:%M:%S +0000 %Y")
        .ok()
        .map(|it| it.into())
}

fn millis_to_utc(millis: i64) -> Option<DateTime<Utc>> {
    match Utc.timestamp_millis_opt(millis) {
        chrono::LocalResult::Single(v) => Some(v),
        _ => None,
    }
}

fn from_v1_indicies(src: &serde_json::Value) -> (Option<i64>, Option<i64>) {
    match src["indices"].as_array() {
        Some(indices) => (
            indices.get(0).and_then(|it| it.as_i64()),
            indices.get(1).and_then(|it| it.as_i64()),
        ),
        None => (None, None),
    }
}

fn from_v1_menthions(src: &serde_json::Value) -> Vec<crate::responses::mentions::Mentions> {
    match src.as_array() {
        Some(user_mentions) => user_mentions
            .iter()
            .map(|it| {
                let indices = from_v1_indicies(it);
                crate::responses::mentions::Mentions {
                    username: it["screen_name"].as_str().map(|it| it.to_owned()),
                    id: it["id_str"].as_str().map(|it| it.to_owned()),
                    start: indices.0,
                    end: indices.1,
                    ..Default::default()
                }
            })
            .collect(),
        None => vec![],
    }
}

fn from_v1_hashtags(src: &serde_json::Value) -> Vec<crate::responses::hashtags::Hashtags> {
    match src.as_array() {
        Some(user_mentions) => user_mentions
            .iter()
            .map(|it| {
                let indices = from_v1_indicies(it);
                crate::responses::hashtags::Hashtags {
                    tag: it["text"].as_str().map(|it| it.to_owned()),
                    start: indices.0,
                    end: indices.1,
                    ..Default::default()
                }
            })
            .collect(),
        None => vec![],
    }
}

fn from_v1_urls(src: &serde_json::Value) -> Vec<crate::responses::urls::Urls> {
    match src.as_array() {
        Some(targets) => targets
            .iter()
            .map(|it| {
                let indices = from_v1_indicies(it);
                crate::responses::urls::Urls {
                    url: it["url"].as_str().map(|it| it.to_owned()),
                    display_url: it["display_url"].as_str().map(|it| it.to_owned()),
                    expanded_url: it["expanded_url"].as_str().map(|it| it.to_owned()),
                    title: it["unwound"]["title"].as_str().map(|it| it.to_owned()),
                    description: it["unwound"]["description"]
                        .as_str()
                        .map(|it| it.to_owned()),
                    status: it["unwound"]["status"].as_i64(),
                    start: indices.0,
                    end: indices.1,
                    ..Default::default()
                }
            })
            .collect(),
        None => vec![],
    }
}

fn from_v1_edit_controls(
    src: &serde_json::Value,
) -> Option<crate::responses::edit_controls::EditControls> {
    if src.is_object() {
        Some(crate::responses::edit_controls::EditControls {
            editable_until: millis_to_utc(src["editable_until_ms"].as_i64().unwrap_or_default()),
            edits_remaining: src["edits_remaining"].as_i64(),
            ..Default::default()
        })
    } else {
        None
    }
}

fn from_v1_edit_history_tweet_ids(src: &serde_json::Value) -> Vec<String> {
    if let Some(value) = src.as_array() {
        value
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_owned())
            .collect()
    } else {
        vec![]
    }
}

fn from_v1_media(
    src: &serde_json::Value,
) -> (
    Vec<crate::responses::urls::Urls>,
    Option<crate::responses::attachments::Attachments>,
) {
    let mut media_keys = vec![];

    let urls = if let Some(targets) = src.as_array() {
        targets
            .iter()
            .map(|it| {
                let indices = from_v1_indicies(it);
                let media_type = match it["type"].as_str() {
                    Some("photo") => "3",
                    _ => "0",
                };
                let media_key = format!(
                    "{}_{}",
                    media_type,
                    it["id_str"].as_str().unwrap_or_default()
                );
                media_keys.push(media_key.clone());
                crate::responses::urls::Urls {
                    url: it["url"].as_str().map(|it| it.to_owned()),
                    display_url: it["display_url"].as_str().map(|it| it.to_owned()),
                    expanded_url: it["expanded_url"].as_str().map(|it| it.to_owned()),
                    media_key: Some(media_key),
                    start: indices.0,
                    end: indices.1,
                    ..Default::default()
                }
            })
            .collect()
    } else {
        vec![]
    };

    let attachments = if !media_keys.is_empty() {
        Some(crate::responses::attachments::Attachments {
            media_keys: Some(media_keys),
            ..Default::default()
        })
    } else {
        None
    };
    (urls, attachments)
}

fn from_v1_entities(
    src: &serde_json::Value,
) -> (
    Option<crate::responses::entities::Entities>,
    Option<crate::responses::attachments::Attachments>,
) {
    if src.is_object() {
        let (mut urls1, attachments) = from_v1_media(&src["media"]);
        let mut urls2 = from_v1_urls(&src["urls"]);
        urls1.append(&mut urls2);
        (
            Some(crate::responses::entities::Entities {
                mentions: Some(from_v1_menthions(&src["user_mentions"])),
                hashtags: Some(from_v1_hashtags(&src["hashtags"])),
                urls: Some(urls1),
                ..Default::default()
            }),
            attachments,
        )
    } else {
        (None, None)
    }
}

fn from_v1_public_metrics(
    src: &serde_json::Value,
) -> Option<crate::responses::public_metrics::PublicMetrics> {
    Some(crate::responses::public_metrics::PublicMetrics {
        retweet_count: src["retweet_count"].as_i64(),
        quote_count: src["quote_count"].as_i64(),
        reply_count: src["reply_count"].as_i64(),
        like_count: src["favorite_count"].as_i64(),
        ..Default::default()
    })
}

fn from_v1_users(src: &serde_json::Value) -> crate::responses::users::Users {
    let public_metrics = crate::responses::users::PublicMetrics {
        followers_count: src["followers_count"].as_i64(),
        following_count: src["friends_count"].as_i64(),
        tweet_count: src["statuses_count"].as_i64(),
        listed_count: src["listed_count"].as_i64(),
        ..Default::default()
    };

    crate::responses::users::Users {
        created_at: str_to_utc(src["created_at"].as_str().unwrap_or_default()),
        description: src["description"].as_str().map(|it| it.to_owned()),
        id: src["id_str"].as_str().unwrap_or_default().to_owned(),
        location: src["location"].as_str().map(|it| it.to_owned()),
        name: src["name"].as_str().unwrap_or_default().to_owned(),
        profile_image_url: src["profile_image_url_https"]
            .as_str()
            .map(|it| it.to_owned()),
        protected: src["protected"].as_bool(),
        public_metrics: Some(public_metrics),
        url: src["url"].as_str().map(|it| it.to_owned()),
        username: src["screen_name"].as_str().unwrap_or_default().to_owned(),
        verified: src["verified"].as_bool(),
        verified_type: src["verified_type"].as_str().map(|it| it.to_owned()),
        ..Default::default()
    }
}

fn from_v1_tweets(
    src: &serde_json::Value,
    tweet_map: &mut HashMap<String, crate::responses::tweets::Tweets>,
    user_map: &mut HashMap<String, crate::responses::users::Users>,
) -> Option<crate::responses::tweets::Tweets> {
    if src.is_object() {
        let (entities, attachments) = from_v1_entities(&src["entities"]);
        let mut data = crate::responses::tweets::Tweets {
            id: src["id_str"].as_str().unwrap_or_default().to_owned(),
            text: src["text"].as_str().unwrap_or_default().to_owned(),
            attachments,
            source: src["source"].as_str().map(|it| it.to_owned()),
            author_id: src["user"]["id_str"].as_str().map(|it| it.to_owned()),
            conversation_id: src["edit_history"]["initial_tweet_id"]
                .as_str()
                .map(|it| it.to_owned()),
            created_at: str_to_utc(src["created_at"].as_str().unwrap_or_default()),
            edit_controls: from_v1_edit_controls(&src["edit_controls"]),
            edit_history_tweet_ids: from_v1_edit_history_tweet_ids(
                &src["edit_history"]["edit_tweet_ids"],
            ),
            entities,
            in_reply_to_user_id: src["in_reply_to_user_id_str"]
                .as_str()
                .map(|it| it.to_owned()),
            lang: src["lang"].as_str().map(|it| it.to_owned()),
            possibly_sensitive: src["possibly_sensitive"].as_bool(),
            public_metrics: from_v1_public_metrics(src),
            ..Default::default()
        };

        // TODO : attachments

        // TODO : geo

        // TODO : note_tweet

        // TODO : withheld

        let mut referenced_tweets = vec![];

        if src["retweeted_status"].is_object() {
            if let Some(tweet) = from_v1_tweets(&src["retweeted_status"], tweet_map, user_map) {
                referenced_tweets.push(crate::responses::referenced_tweets::ReferencedTweets {
                    id: Some(tweet.id.clone()),
                    r#type: Some(crate::responses::referenced_tweets::Type::Retweeted),
                    ..Default::default()
                });
                tweet_map.insert(tweet.id.clone(), tweet);
            }
        }

        if src["quoted_status"].is_object() {
            if let Some(tweet) = from_v1_tweets(&src["quoted_status"], tweet_map, user_map) {
                referenced_tweets.push(crate::responses::referenced_tweets::ReferencedTweets {
                    id: Some(tweet.id.clone()),
                    r#type: Some(crate::responses::referenced_tweets::Type::Quoted),
                    ..Default::default()
                });
                tweet_map.insert(tweet.id.clone(), tweet);
            }
        }

        // TODO : replay_status

        if !referenced_tweets.is_empty() {
            data.referenced_tweets = Some(referenced_tweets);
        }

        let user = from_v1_users(&src["user"]);
        user_map.insert(user.id.clone(), user);

        Some(data)
    } else {
        None
    }
}

impl TweetModel {
    pub fn from_v1(src: &serde_json::Value) -> Self {
        let mut tweet_map = HashMap::new();
        let mut user_map = HashMap::new();
        let data = from_v1_tweets(src, &mut tweet_map, &mut user_map);
        Self {
            data,
            includes: Some(crate::responses::includes::Includes {
                tweets: Some(tweet_map.into_values().collect()),
                users: Some(user_map.into_values().collect()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
