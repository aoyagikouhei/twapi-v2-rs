use axum::{
    extract::Query, response::{Html, IntoResponse}, routing::get, Json, Router
};
use std::collections::HashMap;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use twapi_v2::{
    api::{get_2_users_me, BearerAuthentication},
    oauth::{TwitterOauth, TwitterScope},
};

pub const PKCE_VERIFIER: &str = "pkce_verifier";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/oauth", get(oauth))
        .route("/", get(root))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn oauth_client() -> TwitterOauth {
    TwitterOauth::new(
        &std::env::var("API_KEY_CODE").unwrap(),
        &std::env::var("API_SECRET_CODE").unwrap(),
        &std::env::var("CALLBACK_URL").unwrap(),
        TwitterScope::all(),
    )
    .unwrap()
}

async fn root(cookies: Cookies) -> impl IntoResponse {
    let oauth = oauth_client();
    let res = oauth.oauth_url();
    cookies.add(Cookie::new(PKCE_VERIFIER, res.pkce_verifier.clone()));
    Html(format!("<a href='{}'>oauth<a>", res.oauth_url)).into_response()
}

async fn oauth(Query(params): Query<HashMap<String, String>>, cookies: Cookies) -> impl IntoResponse {
    let pkce = cookies.get(PKCE_VERIFIER).unwrap();
    let oauth = oauth_client();
    let res = oauth
        .token(pkce.value(), params.get("code").unwrap(), None)
        .await
        .unwrap();
    println!("{:?}", res);
    let auth = BearerAuthentication::new(res.access_token);
    let me = get_2_users_me::Api::all().execute(&auth).await.unwrap();
    Json(me.0).into_response()
}
