use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use std::time::Duration;
use thiserror::Error;

pub enum TwitterScope {
    TweetRead,
    TweetWrite,
    TweetModerateWrite,
    UsersRead,
    FollowsRead,
    FollowsWrite,
    OfflineAccess,
    SpaceRead,
    MuteRead,
    MuteWrite,
    LikeRead,
    LikeWrite,
    ListRead,
    ListWrite,
    BlockRead,
    BlockWrite,
    BookmarkRead,
    BookmarkWrite,
    DmRead,
    DmWrite,
    MediaWrite,
}

impl TwitterScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::TweetRead,
            Self::TweetWrite,
            Self::TweetModerateWrite,
            Self::UsersRead,
            Self::FollowsRead,
            Self::FollowsWrite,
            Self::OfflineAccess,
            Self::SpaceRead,
            Self::MuteRead,
            Self::MuteWrite,
            Self::LikeRead,
            Self::LikeWrite,
            Self::ListRead,
            Self::ListWrite,
            Self::BlockRead,
            Self::BlockWrite,
            Self::BookmarkRead,
            Self::BookmarkWrite,
            Self::DmRead,
            Self::DmWrite,
            Self::MediaWrite,
        ]
    }
}

impl std::fmt::Display for TwitterScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TweetRead => write!(f, "tweet.read"),
            Self::TweetWrite => write!(f, "tweet.write"),
            Self::TweetModerateWrite => write!(f, "tweet.moderate.write"),
            Self::UsersRead => write!(f, "users.read"),
            Self::FollowsRead => write!(f, "follows.read"),
            Self::FollowsWrite => write!(f, "follows.write"),
            Self::OfflineAccess => write!(f, "offline.access"),
            Self::SpaceRead => write!(f, "space.read"),
            Self::MuteRead => write!(f, "mute.read"),
            Self::MuteWrite => write!(f, "mute.write"),
            Self::LikeRead => write!(f, "like.read"),
            Self::LikeWrite => write!(f, "like.write"),
            Self::ListRead => write!(f, "list.read"),
            Self::ListWrite => write!(f, "list.write"),
            Self::BlockRead => write!(f, "block.read"),
            Self::BlockWrite => write!(f, "block.write"),
            Self::BookmarkRead => write!(f, "bookmark.read"),
            Self::BookmarkWrite => write!(f, "bookmark.write"),
            Self::DmRead => write!(f, "dm.read"),
            Self::DmWrite => write!(f, "dm.write"),
            Self::MediaWrite => write!(f, "media.write"),
        }
    }
}

const AUTH_URL: &str = "https://x.com/i/oauth2/authorize";
const TOKEN_URL: &str = "https://api.x.com/2/oauth2/token";

#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("Url {0}")]
    Url(#[from] oauth2::url::ParseError),

    #[error("Reqwest {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Token {0}")]
    Token(String),
}

#[derive(Debug, Clone)]
pub struct OAuthUrlResult {
    pub oauth_url: String,
    pub pkce_verifier: String,
}

#[derive(Debug, Clone)]
pub struct TokenResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<Duration>,
}

pub struct TwitterOauth {
    client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
    scopes: Vec<Scope>,
}

impl TwitterOauth {
    pub fn new(
        api_key_code: &str,
        api_secret_code: &str,
        callback_url: &str,
        scopes: Vec<TwitterScope>,
    ) -> Result<Self, OAuthError> {
        let redirect_url = RedirectUrl::new(callback_url.to_string())?;
        let scopes: Vec<Scope> = scopes
            .into_iter()
            .map(|it| Scope::new(it.to_string()))
            .collect();
        Ok(Self {
            client_id: ClientId::new(api_key_code.to_owned()),
            client_secret: ClientSecret::new(api_secret_code.to_owned()),
            auth_url: AuthUrl::new(AUTH_URL.to_owned())?,
            token_url: TokenUrl::new(TOKEN_URL.to_owned())?,
            redirect_url,
            scopes,
        })
    }

    pub fn oauth_url(&self) -> OAuthUrlResult {
        self.oauth_url_with_state(None)
    }

    pub fn oauth_url_with_state(&self, state: Option<String>) -> OAuthUrlResult {
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
        let csrf_token = match state {
            Some(ref state_value) => CsrfToken::new(state_value.clone()),
            None => CsrfToken::new_random(),
        };

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone());

        let (auth_url, _csrf_token) = client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .authorize_url(|| csrf_token)
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
        twapi_options: Option<&TwapiOptions>,
    ) -> Result<TokenResult, OAuthError> {
        let pkce_verifier = oauth2::PkceCodeVerifier::new(pkce_verifier_str.to_owned());

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone());

        let mut client_builder = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none());
        
        if let Some(twapi_options) = twapi_options {
            client_builder = client_builder.timeout(twapi_options.timeout);
        }

        let http_client = client_builder.build()?;

        let token = client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| OAuthError::Token(format!("{:?}", e)))?;
        Ok(TokenResult {
            access_token: token.access_token().secret().to_string(),
            refresh_token: token.refresh_token().map(|it| it.secret().to_string()),
            expires_in: token.expires_in(),
        })
    }
}
