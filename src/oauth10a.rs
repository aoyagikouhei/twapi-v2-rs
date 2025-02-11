use crate::api::Authentication;
use base64::{engine::general_purpose, Engine as _};
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use rand::distr::{Alphanumeric, SampleString};
use reqwest::RequestBuilder;
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub struct OAuthAuthentication {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl OAuthAuthentication {
    pub fn new<T: Into<String>>(
        consumer_key: T,
        consumer_secret: T,
        access_key: T,
        access_secret: T,
    ) -> Self {
        Self {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            access_key: access_key.into(),
            access_secret: access_secret.into(),
        }
    }
}

impl Authentication for OAuthAuthentication {
    fn execute(
        &self,
        builder: RequestBuilder,
        method: &str,
        uri: &str,
        options: &[(&str, &str)],
    ) -> RequestBuilder {
        let auth = oauth1_authorization_header(
            &self.consumer_key,
            &self.consumer_secret,
            &self.access_key,
            &self.access_secret,
            method,
            uri,
            &options.to_vec(),
        );
        builder.header(reqwest::header::AUTHORIZATION, auth)
    }
}

fn oauth1_authorization_header(
    consumer_key: &str,
    consumer_secret: &str,
    access_token: &str,
    access_token_secret: &str,
    method: &str,
    uri: &str,
    options: &Vec<(&str, &str)>,
) -> String {
    let res = calc_oauth_header(
        &format!("{}&{}", consumer_secret, access_token_secret),
        consumer_key,
        &vec![("oauth_token", access_token)],
        method,
        uri,
        options,
    );
    format!("OAuth {}", res)
}

fn calc_oauth_header(
    sign_key: &str,
    consumer_key: &str,
    header_options: &Vec<(&str, &str)>,
    method: &str,
    uri: &str,
    options: &Vec<(&str, &str)>,
) -> String {
    let mut param0: Vec<(&str, String)> = vec![
        ("oauth_consumer_key", String::from(consumer_key)),
        ("oauth_nonce", nonce()),
        ("oauth_signature_method", String::from("HMAC-SHA1")),
        ("oauth_timestamp", timestamp()),
        ("oauth_version", String::from("1.0")),
    ];
    for header_option in header_options {
        param0.push((header_option.0, encode(header_option.1)));
    }
    let mut param1 = param0.clone();
    for option in options {
        param1.push((option.0, encode(option.1)));
    }
    param1.sort();
    let parameter = make_query(&param1, "&");
    let base = format!("{}&{}&{}", method, encode(uri), encode(&parameter));
    let mut param2 = param0.clone();
    param2.push(("oauth_signature", encode(&sign(&base, sign_key))));
    make_query(&param2, ", ")
}

fn nonce() -> String {
    let mut rng = &mut rand::rng();
    Alphanumeric.sample_string(&mut rng, 32)
}

fn timestamp() -> String {
    format!("{}", Utc::now().timestamp())
}

pub fn encode(s: &str) -> String {
    // Twitter API URL encode space is %20 not +
    form_urlencoded::byte_serialize(s.as_bytes())
        .collect::<String>()
        .replace('+', "%20")
        .replace('*', "%2A")
        .replace("%7E", "~")
}

fn sign(base: &str, key: &str) -> String {
    let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(base.as_bytes());
    let result = mac.finalize();
    general_purpose::STANDARD.encode(result.into_bytes())
}

fn make_query(list: &Vec<(&str, String)>, separator: &str) -> String {
    let mut result = String::from("");
    for item in list {
        if !result.is_empty() {
            result.push_str(separator);
        }
        result.push_str(&format!("{}={}", item.0, item.1));
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::oauth10a::oauth1_authorization_header;
    #[test]
    fn it_oauth2_authorization_header() {
        println!(
            "{}",
            oauth1_authorization_header("a", "b", "c", "d", "GET", "http://localhost", &vec![])
        );
    }
}
