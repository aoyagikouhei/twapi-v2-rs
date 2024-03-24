use chrono::prelude::*;
use std::collections::HashMap;

const X_ACCESS_LEVEL: &str = "x-access-level";
const X_APP_LIMIT_24HOUR_LIMIT: &str = "x-app-limit-24hour-limit";
const X_APP_LIMIT_24HOUR_RESET: &str = "x-app-limit-24hour-reset";
const X_APP_LIMIT_24HOUR_REMAINING: &str = "x-app-limit-24hour-remaining";
const X_CONNECTION_HASH: &str = "x-connection-hash";
const X_CONTENT_TYPE_OPTIONS: &str = "x-content-type-options";
const X_FRAME_OPTIONS: &str = "x-frame-options";
const X_RATE_LIMIT_LIMIT: &str = "x-rate-limit-limit";
const X_RATE_LIMIT_RESET: &str = "x-rate-limit-reset";
const X_RATE_LIMIT_REMAINING: &str = "x-rate-limit-remaining";
const X_RESPONSE_TIME: &str = "x-response-time";
const X_TRANSACTION_ID: &str = "x-transaction-id";
const X_USER_LIMIT_24HOUR_LIMIT: &str = "x-user-limit-24hour-limit";
const X_USER_LIMIT_24HOUR_RESET: &str = "x-user-limit-24hour-reset";
const X_USER_LIMIT_24HOUR_REMAINING: &str = "x-user-limit-24hour-remaining";
const X_XSS_PROTECTION: &str = "x-xss-protection";

const KEYS: [&str; 16] = [
    X_ACCESS_LEVEL,
    X_APP_LIMIT_24HOUR_LIMIT,
    X_APP_LIMIT_24HOUR_RESET,
    X_APP_LIMIT_24HOUR_REMAINING,
    X_CONNECTION_HASH,
    X_CONTENT_TYPE_OPTIONS,
    X_FRAME_OPTIONS,
    X_RATE_LIMIT_LIMIT,
    X_RATE_LIMIT_RESET,
    X_RATE_LIMIT_REMAINING,
    X_RESPONSE_TIME,
    X_TRANSACTION_ID,
    X_USER_LIMIT_24HOUR_LIMIT,
    X_USER_LIMIT_24HOUR_RESET,
    X_USER_LIMIT_24HOUR_REMAINING,
    X_XSS_PROTECTION,
];

#[derive(Debug, Clone)]
pub struct Headers {
    pub x_access_level: Option<String>,
    pub x_app_limit_24hour_limit: Option<u64>,
    pub x_app_limit_24hour_reset: Option<DateTime<Utc>>,
    pub x_app_limit_24hour_remaining: Option<u64>,
    pub x_connection_hash: Option<String>,
    pub x_content_type_options: Option<String>,
    pub x_frame_options: Option<String>,
    pub x_rate_limit_limit: Option<u64>,
    pub x_rate_limit_reset: Option<DateTime<Utc>>,
    pub x_rate_limit_remaining: Option<u64>,
    pub x_response_time: Option<u64>,
    pub x_transaction_id: Option<String>,
    pub x_user_limit_24hour_limit: Option<u64>,
    pub x_user_limit_24hour_reset: Option<DateTime<Utc>>,
    pub x_user_limit_24hour_remaining: Option<u64>,
    pub x_xss_protection: Option<u64>,
    pub extra: HashMap<String, String>,
}

impl Headers {
    pub fn new(header: &reqwest::header::HeaderMap) -> Self {
        let mut extra = HashMap::new();
        for (name, value) in header.iter() {
            if !KEYS.contains(&name.as_str()) {
                if let Ok(value) = value.to_str() {
                    extra.insert(name.as_str().to_string(), value.to_string());
                }
            }
        }
        Self {
            x_access_level: get_string_value(header, X_ACCESS_LEVEL),
            x_app_limit_24hour_limit: get_integer_value(header, X_APP_LIMIT_24HOUR_LIMIT),
            x_app_limit_24hour_reset: get_unixtimestamp_value(header, X_APP_LIMIT_24HOUR_RESET),
            x_app_limit_24hour_remaining: get_integer_value(header, X_APP_LIMIT_24HOUR_REMAINING),
            x_connection_hash: get_string_value(header, X_CONNECTION_HASH),
            x_content_type_options: get_string_value(header, X_CONTENT_TYPE_OPTIONS),
            x_frame_options: get_string_value(header, X_FRAME_OPTIONS),
            x_rate_limit_limit: get_integer_value(header, X_RATE_LIMIT_LIMIT),
            x_rate_limit_reset: get_unixtimestamp_value(header, X_RATE_LIMIT_RESET),
            x_rate_limit_remaining: get_integer_value(header, X_RATE_LIMIT_REMAINING),
            x_response_time: get_integer_value(header, X_RESPONSE_TIME),
            x_transaction_id: get_string_value(header, X_TRANSACTION_ID),
            x_user_limit_24hour_limit: get_integer_value(header, X_USER_LIMIT_24HOUR_LIMIT),
            x_user_limit_24hour_reset: get_unixtimestamp_value(header, X_USER_LIMIT_24HOUR_RESET),
            x_user_limit_24hour_remaining: get_integer_value(header, X_USER_LIMIT_24HOUR_REMAINING),
            x_xss_protection: get_integer_value(header, X_XSS_PROTECTION),
            extra,
        }
    }
}

impl std::fmt::Display for Headers {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_integer_value(header: &reqwest::header::HeaderMap, key: &str) -> Option<u64> {
    header.get(key)?.to_str().ok()?.parse().ok()
}

fn get_string_value(header: &reqwest::header::HeaderMap, key: &str) -> Option<String> {
    header.get(key)?.to_str().ok().map(|it| it.to_string())
}

fn get_unixtimestamp_value(
    header: &reqwest::header::HeaderMap,
    key: &str,
) -> Option<DateTime<Utc>> {
    let res = get_integer_value(header, key)?;
    Utc.timestamp_opt(res as i64, 0).single()
}
