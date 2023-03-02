use axum::{Extension, Json, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{infra::{App, Service, response}, modules::{auth, jwt::{RawJwtAccessToken, RawJwtRefreshToken}}};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String
}

struct RegisterResponse {
    access_token: RawJwtAccessToken,
    refresh_token: RawJwtRefreshToken
}

impl IntoResponse for RegisterResponse {
    fn into_response(self) -> axum::response::Response {
        let data = json!({
            "access_token": self.access_token,
            "refresh_token": self.refresh_token
        });

        response::created(data)
    }
}

impl From<RegisterRequest> for auth::Register {
    fn from(RegisterRequest { username, email, password }: RegisterRequest) -> Self {
        auth::Register { username, email, password }
    }
}

pub async fn register(
    Extension(app): Extension<App>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
     let register = app.resolver.register_service();

     let register_input = request.into();
     register.execute(register_input).await.map(|(access_token, refresh_token)| {
         RegisterResponse { 
             access_token: access_token.raw, 
             refresh_token: refresh_token.raw
         }
     })
}
