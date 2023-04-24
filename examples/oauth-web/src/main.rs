use axum::{
    http::Uri,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use std::{collections::HashMap, net::SocketAddr};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use twapi_v2::{
    api::get_2_users_me,
    oauth::{TwitterOauth, TwitterScope},
};
use url::Url;

pub const PKCE_VERIFIER: &str = "pkce_verifier";

#[tokio::main]
async fn main() {
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
    )
    .unwrap()
}

async fn root(cookies: Cookies) -> impl IntoResponse {
    let oauth = oauth_client();
    let res = oauth.oauth_url();
    cookies.add(Cookie::new(PKCE_VERIFIER, res.pkce_verifier.clone()));
    Html(format!("<a href='{}'>oauth<a>", res.oauth_url)).into_response()
}

async fn oauth(uri: Uri, cookies: Cookies) -> impl IntoResponse {
    let url = Url::parse(&format!("http://localhost:3000{}", uri)).unwrap();
    let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let pkce = cookies.get(PKCE_VERIFIER).unwrap();
    let oauth = oauth_client();
    let res = oauth
        .token(pkce.value(), hash_query.get("code").unwrap())
        .await
        .unwrap();
    println!("{:?}", res);
    let me = get_2_users_me::Api::all(&res.access_token)
        .execute()
        .await
        .unwrap();
    Json(me.0).into_response()
}
