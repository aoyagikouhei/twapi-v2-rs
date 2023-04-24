use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::time::Duration;
use thiserror::Error;

pub const TWEET_READ: &str = "tweet.read";
pub const TWEET_WRITE: &str = "tweet.write";
pub const TWEET_MODERATE_WRITE: &str = "tweet.moderate.write";
pub const USERS_READ: &str = "users.read";
pub const FOLLOWS_READ: &str = "follows.read";
pub const FOLLOWS_WRITE: &str = "follows.write";
pub const OFFLINE_ACCESS: &str = "offline.access";
pub const SPACE_READ: &str = "space.read";
pub const MUTE_READ: &str = "mute.read";
pub const MUTE_WRITE: &str = "mute.write";
pub const LIKE_READ: &str = "like.read";
pub const LIKE_WRITE: &str = "like.write";
pub const LIST_READ: &str = "list.read";
pub const LIST_WRITE: &str = "list.write";
pub const BLOCK_READ: &str = "block.read";
pub const BLOCK_WRITE: &str = "block.write";
pub const BOOKMARK_READ: &str = "bookmark.read";
pub const BOOKMARK_WRITE: &str = "bookmark.write";

const AUTH_URL: &str = "https://twitter.com/i/oauth2/authorize";
const TOKEN_URL: &str = "https://api.twitter.com/2/oauth2/token";

#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("Url {0}")]
    Url(#[from] url::ParseError),

    #[error("Token {0}")]
    Token(String),
}

pub struct OAuthUrlResult {
    pub oauth_url: String,
    pub pkce_verifier: String,
}

pub struct TokenResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<Duration>,
}

pub struct TwitterOauth {
    basic_client: BasicClient,
    redirect_url: RedirectUrl,
    scopes: Vec<Scope>,
}

impl TwitterOauth {
    pub fn new(
        api_key_code: &str,
        api_secret_code: &str,
        callback_url: &str,
        scopes: Vec<&str>,
    ) -> Result<Self, OAuthError> {
        let basic_client = BasicClient::new(
            ClientId::new(api_key_code.to_owned()),
            Some(ClientSecret::new(api_secret_code.to_owned())),
            AuthUrl::new(AUTH_URL.to_owned())?,
            Some(TokenUrl::new(TOKEN_URL.to_owned())?),
        );
        let redirect_url = RedirectUrl::new(callback_url.to_string())?;
        let scopes: Vec<Scope> = scopes
            .into_iter()
            .map(|it| Scope::new(it.to_owned()))
            .collect();
        Ok(Self {
            basic_client,
            redirect_url,
            scopes,
        })
    }

    pub fn oauth_url(&self) -> OAuthUrlResult {
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
        let (auth_url, _csrf_token) = self
            .basic_client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .set_pkce_challenge(pkce_challenge)
            .url();

        OAuthUrlResult {
            oauth_url: auth_url.to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
        }
    }

    pub async fn token(
        &self,
        pkce_verifier_str: &str,
        code: &str,
    ) -> Result<TokenResult, OAuthError> {
        let pkce_verifier = oauth2::PkceCodeVerifier::new(pkce_verifier_str.to_owned());

        let token = self
            .basic_client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuthError::Token(format!("{:?}", e)))?;
        Ok(TokenResult {
            access_token: token.access_token().secret().to_string(),
            refresh_token: token.refresh_token().map(|it| it.secret().to_string()),
            expires_in: token.expires_in(),
        })
    }
}
