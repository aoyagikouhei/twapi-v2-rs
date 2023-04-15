use chrono::prelude::*;

const LIMIT: &str = "x-rate-limit-limit";
const REMAINING: &str = "x-rate-limit-remaining";
const RESET: &str = "x-rate-limit-reset";

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub limit: i64,
    pub remaining: i64,
    pub reset: DateTime<Utc>,
}

impl RateLimit {
    pub fn new(header: &reqwest::header::HeaderMap) -> Option<Self> {
        if header.get(LIMIT).is_some() {
            Some(RateLimit {
                limit: get_value(header, LIMIT),
                remaining: get_value(header, REMAINING),
                reset: Utc
                    .timestamp_opt(get_value(header, RESET), 0)
                    .single()
                    .unwrap_or_default(),
            })
        } else {
            None
        }
    }
}

impl std::fmt::Display for RateLimit {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_value(header: &reqwest::header::HeaderMap, key: &str) -> i64 {
    let res = if let Some(res) = header.get(key) {
        res
    } else {
        return 0;
    };
    let res = if let Ok(res) = res.to_str() {
        res
    } else {
        return 0;
    };
    res.parse().unwrap_or_default()
}