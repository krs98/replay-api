use async_trait::async_trait;
use auto_impl::auto_impl;
use serde::Deserialize;

use crate::{modules::{users::{OAuthAccessToken, Username, Email}, auth::{OAuthProvider, OAuthCode}}, Error};

pub struct OAuthLoginResponse {
    pub access_token: OAuthAccessToken,
}

#[derive(Deserialize)]
pub struct OAuthUserResponse {
    pub username: Username,
    pub email: Option<Email>
}

#[async_trait]
#[auto_impl(&, Arc)]
pub trait OAuthStore {
    async fn login(&self, code: OAuthCode) -> Result<OAuthLoginResponse, Error>;
    async fn user(&self, access_token: OAuthAccessToken) -> Result<OAuthUserResponse, Error>;
}

pub type DynOAuthStore = Box<dyn OAuthStore + Send + Sync>;

#[auto_impl(&, Arc)]
pub trait AsOAuthStore {
    fn as_oauth_store(&self, provider: &OAuthProvider) -> &DynOAuthStore;
}
