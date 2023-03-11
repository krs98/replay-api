use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{modules::{users::{OAuthAccessToken, Username, Email}, auth::OAuthCode}, Error};

use super::{OAuthStore, OAuthLoginResponse, OAuthUserResponse};

pub struct GitlabOAuthStore {
    // oauth_config: config::GitlabOAuth
}

impl GitlabOAuthStore {
    pub fn new(/*oauth_config: config::GitlabOAuth*/) -> Self {
        GitlabOAuthStore { /*oauth_config*/ }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GitlabLoginResponse {
    pub access_token: OAuthAccessToken
}

impl std::convert::From<GitlabLoginResponse> for OAuthLoginResponse {
    fn from(value: GitlabLoginResponse) -> Self {
        OAuthLoginResponse {
            access_token: value.access_token
        }
    }
}

#[derive(Deserialize)]
pub struct GitlabUserResponse {
    pub username: Username,
    pub email: Option<Email>,
}

impl std::convert::From<GitlabUserResponse> for OAuthUserResponse {
    fn from(value: GitlabUserResponse) -> Self {
        OAuthUserResponse { 
            username: value.username,
            email: value.email
        }
    }
}

#[async_trait]
impl OAuthStore for GitlabOAuthStore {
    async fn login(&self, code: OAuthCode) -> Result<OAuthLoginResponse, Error> {
        todo!()
    }

    async fn user(&self, access_token: OAuthAccessToken) -> Result<OAuthUserResponse, Error> {
        todo!()
    }
}

