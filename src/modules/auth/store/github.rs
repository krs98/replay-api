use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tracing::debug;

use crate::{infra::config, modules::{auth::OAuthCode, users::{OAuthAccessToken, Username, Email}}, Error};

use super::{OAuthLoginResponse, OAuthStore, OAuthUserResponse};

pub struct GithubOAuthStore {
    reqwest: reqwest::Client,
    oauth_config: config::GithubOAuth
}

impl GithubOAuthStore {
    pub fn new(oauth_config: config::GithubOAuth) -> Self {
        let reqwest = reqwest::Client::new();
        GithubOAuthStore { reqwest, oauth_config }
    }
}

#[derive(Serialize)]
struct GithubLoginRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: OAuthCode
}

#[derive(Serialize, Deserialize)]
pub struct GithubLoginResponse {
    pub access_token: OAuthAccessToken
}

impl std::convert::From<GithubLoginResponse> for OAuthLoginResponse {
    fn from(value: GithubLoginResponse) -> Self {
        OAuthLoginResponse {
            access_token: value.access_token 
        }
    }
}

#[derive(Deserialize)]
pub struct GithubUserResponse {
    #[serde(rename = "login")]
    pub username: Username,
    pub email: Option<Email>
}

impl std::convert::From<GithubUserResponse> for OAuthUserResponse {
    fn from(value: GithubUserResponse) -> Self {
        OAuthUserResponse { 
            username: value.username,
            email: value.email
        } 
    }
}

#[async_trait]
impl OAuthStore for GithubOAuthStore {
    async fn login(&self, code: OAuthCode) -> Result<OAuthLoginResponse, Error> {
        let request = GithubLoginRequest { 
            client_id: self.oauth_config.client_id.as_str(),
            client_secret: self.oauth_config.client_secret.as_str(),
            code: code.clone(),
        };

        let response: String = self.reqwest
            .post("https://github.com/login/oauth/access_token")
            .header("accept", "application/json")
            .json(&request)
            .send()
            .await?
            .text()
            .await?;

        debug!("{} {:?}", response, code);

        let response: GithubLoginResponse = serde_json::from_str(response.as_str()).map_err(|err| {
            Error::Internal
        })?;

        Ok(response.into())
    }

    async fn user(&self, access_token: OAuthAccessToken) -> Result<OAuthUserResponse, Error> {
        let response = self.reqwest
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token.into_inner()))
            .header("User-Agent", "Replay")
            .send()
            .await?
            .text()
            .await?;

        debug!(response);

        let response: GithubUserResponse = serde_json::from_str(response.as_str()).unwrap();

        Ok(response.into())
    }
}

