use axum::{response::IntoResponse, Extension};
use serde_json::json;
use tracing::info;

use crate::{infra::{App, Service, response}, modules::{users::User, auth::Me}, api::extractors::ExtractJwtAccessToken};

struct MeResponse {
    user: User
}

impl IntoResponse for MeResponse {
    fn into_response(self) -> axum::response::Response {
        let User { username, email, password: _, created_at } = self.user;
        let data = json!({
            "username": username.into_inner(),
            "email": email.into_inner(),
            "created_at": created_at
        });

        response::ok(data)
    }
}

#[tracing::instrument(
    name = "/auth/me",
    skip(app)
)]
pub async fn me(
    Extension(app): Extension<App>,
    ExtractJwtAccessToken(jwt): ExtractJwtAccessToken
) -> impl IntoResponse {
    let me = app.resolver.me_service();

    let me_input = Me { subject: jwt.claims.sub };
    info!("Executing request");
    me.execute(me_input)
        .await
        .map(|user| MeResponse { user })
}
