pub mod post_media_uploda_init;

const ENV_KEY: &str = "TWAPI_V2_MEDIA_API_PREFIX_API";
const PREFIX_URL_MEDIA: &str = "https://upload.twitter.com";

pub fn clear_prefix_url() {
    std::env::set_var(ENV_KEY, PREFIX_URL_MEDIA);
}

pub fn setup_prefix_url(url: &str) {
    std::env::set_var(ENV_KEY, url);
}

pub(crate) fn make_url<S: AsRef<str>>(post_url: S) -> String {
    let prefix_url = std::env::var(ENV_KEY).unwrap_or(PREFIX_URL_MEDIA.to_owned());
    format!("{}{}", prefix_url, post_url.as_ref())
}
