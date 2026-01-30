use axum::{
    extract::Query, response::{Html, IntoResponse}, routing::get, Json, Router
};
use std::collections::HashMap;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use twapi_v2::{
    api::{get_2_users_me, BearerAuthentication},
};
use twapi_oauth2::x::{XClient, XScope};

// API_KEY_CODE=xxx API_SECRET_CODE=xxx CALLBACK_URL=http://localhost:3000/oauth cargo run

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

fn oauth_client() -> XClient {
    XClient::new(
        &std::env::var("API_KEY_CODE").unwrap(),
        &std::env::var("API_SECRET_CODE").unwrap(),
        &std::env::var("CALLBACK_URL").unwrap(),
        XScope::all(),
    )
}

async fn root(cookies: Cookies) -> impl IntoResponse {
    let oauth = oauth_client();
    let (oauth_url, pkce_verifier) = oauth.authorize_url("state");
    cookies.add(Cookie::new(PKCE_VERIFIER, pkce_verifier));
    Html(format!("<a href='{}'>oauth<a>", oauth_url)).into_response()
}

async fn oauth(Query(params): Query<HashMap<String, String>>, cookies: Cookies) -> impl IntoResponse {
    let pkce = cookies.get(PKCE_VERIFIER).unwrap();
    let oauth = oauth_client();
    let res = oauth
        .token(params.get("code").unwrap(), pkce.value(), )
        .await
        .unwrap();
    println!("{:?}", res);
    let auth = BearerAuthentication::new(res.0.access_token);
    let me = get_2_users_me::Api::all().execute(&auth).await.unwrap();
    Json(me.0).into_response()
}
