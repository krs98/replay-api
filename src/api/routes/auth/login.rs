use axum::{response::IntoResponse, Extension, Json, extract::Query};
use serde::Deserialize;
use serde_json::json;

use crate::{
    infra::{response, App, Service},
    modules::{
        auth::{OAuthProvider, Login, OAuthCode, LoginOutput},
        jwt::{RawJwtAccessToken, RawJwtRefreshToken},
    },
};

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub provider: OAuthProvider
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub code: OAuthCode
}

struct LoginResponse {
    access_token: RawJwtAccessToken,
    refresh_token: RawJwtRefreshToken
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        let data = json!({
            "access_token": self.access_token,
            "refresh_token": self.refresh_token
        });

        response::created(data)
    }
}

impl std::convert::From<LoginOutput> for LoginResponse {
    fn from(LoginOutput { access_token, refresh_token }: LoginOutput) -> Self {
        LoginResponse { 
            access_token: access_token.raw, 
            refresh_token: refresh_token.raw 
        } 
    }
}

#[tracing::instrument(name = "/auth/login", skip(app))]
pub async fn login(
    Extension(app): Extension<App>,
    Query(login_query): Query<LoginQuery>,
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {
    let LoginRequest { code } = login_request;
    let LoginQuery { provider } = login_query;

    let login_service = app.resolver.login_with_github_service();
    let login_input = Login { code, provider };

    login_service.execute(login_input)
        .await
        .map(std::convert::Into::<LoginResponse>::into)
}
