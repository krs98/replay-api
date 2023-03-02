use axum::{Extension, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

use crate::{modules::jwt::{RawJwtRefreshToken, RawJwtAccessToken, RefreshTokens}, api::extractors::ExtractJwtRefreshToken, infra::{App, response, Service}};

#[derive(Debug, Serialize)]
struct RefreshResponse {
    access_token: RawJwtAccessToken,
    refresh_token: RawJwtRefreshToken
}

impl IntoResponse for RefreshResponse {
    fn into_response(self) -> axum::response::Response {
       let data = json!({
           "access_token": self.access_token,
           "refresh_token": self.refresh_token
       }); 

       response::created(data)
    }
}

pub async fn refresh(
    Extension(app): Extension<App>,
    ExtractJwtRefreshToken(refresh_token): ExtractJwtRefreshToken
) -> impl IntoResponse {
    let refresh_tokens_service = app.resolver.refresh_tokens_service();

    let refresh_tokens_input = RefreshTokens { refresh_token };
    refresh_tokens_service
        .execute(refresh_tokens_input)
        .await
        .map(|(access_token, refresh_token)| RefreshResponse {
            access_token: access_token.raw,
            refresh_token: refresh_token.raw
        })
}
