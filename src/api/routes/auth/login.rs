use axum::{response::IntoResponse, Extension, Json, extract::Query};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    infra::{response, App, Service},
    modules::{
        auth::{OAuthProvider, Login, OAuthCode, LoginOutput},
        jwt::{RawJwtAccessToken, RawJwtRefreshToken},
    }, Error,
};

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub provider: OAuthProvider
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub code: OAuthCode
}

pub struct LoginResponse {
    pub access_token: RawJwtAccessToken,
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        let data = json!({
            "access_token": self.access_token,
        });

        response::created(data)
    }
}

impl std::convert::From<LoginOutput> for LoginResponse {
    fn from(LoginOutput { access_token, refresh_token: _ }: LoginOutput) -> Self {
        LoginResponse { 
            access_token: access_token.raw, 
        } 
    }
}

#[tracing::instrument(name = "/auth/login", skip(app))]
pub async fn login(
    Extension(app): Extension<App>,
    cookie_jar: CookieJar,
    Query(login_query): Query<LoginQuery>,
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {
    let LoginRequest { code } = login_request;
    let LoginQuery { provider } = login_query;

    let login_service = app.resolver.login_with_github_service();
    let login_input = Login { code, provider };

    login_service.execute(login_input)
        .await
        .map(|output| {
            let refresh_token = output.refresh_token.raw.0.clone();
            let cookie = Cookie::build("refresh_token", refresh_token)
                .http_only(true)
                .same_site(SameSite::Lax)
                .secure(true)
                .finish();

            let cookie_jar = cookie_jar.add(cookie);
            (cookie_jar, std::convert::Into::<LoginResponse>::into(output))
        })
}
