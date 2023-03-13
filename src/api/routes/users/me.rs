use axum::{Extension, response::IntoResponse};
use axum_extra::extract::CookieJar;
use chrono::{Utc, DateTime};
use serde::Serialize;
use serde_json::json;
use tracing::debug;

use crate::{api::extractors::ExtractJwtAccessToken, infra::{App, response, Service}, modules::users::{GetAuthdUser, Email, GetAuthdUserOutput}, Error};

#[derive(Serialize)]
pub struct MeResponse {
    id: i64,
    username: String,
    email: Option<String>,
    created_at: DateTime<Utc>
}

impl std::convert::From<GetAuthdUserOutput> for MeResponse {
    fn from(GetAuthdUserOutput { user }: GetAuthdUserOutput) -> Self {
        MeResponse { 
            id: user.id.into_inner(), 
            username: user.username.into_inner(), 
            email: user.email.map(Email::into_inner), 
            created_at: user.created_at 
        } 
    }
}

impl IntoResponse for MeResponse {
    fn into_response(self) -> axum::response::Response {
        serde_json::to_value(self).map_or(
            response::internal_error(json!(Error::Internal)), 
            response::ok
        )
    }
}

pub async fn me(
    Extension(app): Extension<App>,
    cookie_jar: CookieJar,
    ExtractJwtAccessToken(jwt): ExtractJwtAccessToken
) -> impl IntoResponse {
    let get_authd_user_service = app.resolver.get_authd_user_service();
    let get_authd_user_input = GetAuthdUser { jwt };

    get_authd_user_service.execute(get_authd_user_input)
        .await
        .map(std::convert::Into::<MeResponse>::into)
}
