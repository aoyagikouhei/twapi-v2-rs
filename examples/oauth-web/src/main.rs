use axum::{
    routing::{get, },
    Router, response::{IntoResponse, Html}, http::Uri,
};
use tower_cookies::{CookieManagerLayer, Cookies, Key, Cookie};
use twapi_v2::oauth::{TwitterOauth, TwitterScope};
use once_cell::sync::OnceCell;
use std::{net::SocketAddr, collections::HashMap};
use url::Url;

const KEY_SOURCE: &str = "0000000000000000000000000000000000000000000000000000000000000000";
pub const PKCE_VERIFIER: &str = "pkce_verifier";
pub static KEY: OnceCell<Key> = OnceCell::new();

#[tokio::main]
async fn main() {
    KEY.set(Key::from(KEY_SOURCE.as_bytes())).ok();

    let app = Router::new()
        .route("/oauth", get(oauth))
        .route("/", get(root))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn oauth_client() -> TwitterOauth {
    TwitterOauth::new(
        &std::env::var("API_KEY_CODE").unwrap(),
        &std::env::var("API_SECRET_CODE").unwrap(),
        "http://localhost:3000/oauth",
        vec![TwitterScope::TweetRead, TwitterScope::UsersRead],
    ).unwrap()
}

async fn root(cookies: Cookies) -> impl IntoResponse {
    let oauth = oauth_client();
    let res = oauth.oauth_url();
    let private = cookies.private(KEY.get().unwrap());
    let url = res.oauth_url.clone();
    private.add(Cookie::new(PKCE_VERIFIER, res.oauth_url));
    Html(format!("<a href='{}'>oauth<a>", url)).into_response()
}

async fn oauth(uri: Uri) -> impl IntoResponse {
    let path_and_query = uri.path_and_query().map(|it| it.as_str()).unwrap_or("/");
    let uri = Uri::builder()
        .scheme("http")
        .authority("localhost:3000")
        .path_and_query(path_and_query)
        .build()
        .unwrap();
    let url = Url::parse(&uri.to_string()).unwrap();
    let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();
    format!("{:?}", hash_query)
}
