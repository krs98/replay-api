use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;

use crate::{infra::{response, App, Service}, modules::{auth, jwt::{RawJwtAccessToken, RawJwtRefreshToken}}};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
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

impl From<LoginRequest> for auth::Login {
    fn from(LoginRequest { username, password }: LoginRequest) -> Self {
        auth::Login { username, password }
    }
}

pub async fn login(
    Extension(app): Extension<App>,
    Json(login_request): Json<LoginRequest>
) -> impl IntoResponse {
    let login_service = app.resolver.login_service();

    let login_input = login_request.into();
    login_service
        .execute(login_input)
        .await
        .map(|(access_token, refresh_token)| LoginResponse { 
            access_token: access_token.raw, 
            refresh_token: refresh_token.raw 
        })
}
