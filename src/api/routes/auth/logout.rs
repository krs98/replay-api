use axum::{Extension, response::IntoResponse};

use crate::{api::extractors::ExtractJwtRefreshToken, infra::{App, Service}, modules::jwt::BlacklistRefreshToken};

pub async fn logout(
    Extension(app): Extension<App>,
    ExtractJwtRefreshToken(refresh_token): ExtractJwtRefreshToken,
) -> impl IntoResponse {
    let blacklist_refresh_token_service = app.resolver.blacklist_refresh_token_service();
    let blacklist_refresh_token_input = BlacklistRefreshToken { refresh_token };
    blacklist_refresh_token_service.execute(blacklist_refresh_token_input).await
}

