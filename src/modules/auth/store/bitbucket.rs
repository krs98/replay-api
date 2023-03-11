use async_trait::async_trait;
use serde::Deserialize;

use crate::{modules::{users::{OAuthAccessToken, Username, Email}, auth::OAuthCode}, Error};

use super::{OAuthLoginResponse, OAuthStore, OAuthUserResponse};

pub struct BitbucketOAuthStore {
    // oauth_config: config::BitbucketOAuth
}

impl BitbucketOAuthStore {
    pub fn new(/*oauth_config: config::BitbucketOAuth*/) -> Self {
        BitbucketOAuthStore { /*oauth_config: config::BitbucketOAuth*/ }
    }
}

pub struct BitbucketLoginResponse {
    pub access_token: OAuthAccessToken
}

impl std::convert::From<BitbucketLoginResponse> for OAuthLoginResponse {
    fn from(value: BitbucketLoginResponse) -> Self {
        OAuthLoginResponse { 
            access_token: value.access_token 
        }
    }
}

#[derive(Deserialize)]
pub struct BitbucketUserResponse {
    pub username: Username,
    pub email: Option<Email>,
}

impl std::convert::From<BitbucketUserResponse> for OAuthUserResponse {
    fn from(value: BitbucketUserResponse) -> Self {
        OAuthUserResponse { 
            username: value.username,
            email: value.email,
        } 
    }
}

#[async_trait]
impl OAuthStore for BitbucketOAuthStore {
    async fn login(&self, code: OAuthCode) -> Result<OAuthLoginResponse, Error> {
        todo!()
    }

    async fn user(&self, access_token: OAuthAccessToken) -> Result<OAuthUserResponse, Error> {
        todo!()
    }
}
