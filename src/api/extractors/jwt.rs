use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::{IntoResponse, Response},
    Extension, TypedHeader,
};

use crate::{
    infra::{App, Service},
    modules::jwt::{
        DecodeAccessToken, DecodeRefreshToken, JwtAccessToken, JwtRefreshToken, RawJwtAccessToken,
        RawJwtRefreshToken,
    },
};

pub struct ExtractJwtAccessToken(pub JwtAccessToken);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractJwtAccessToken
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| err.into_response())?;

        let Extension(app) = Extension::<App>::from_request_parts(parts, state)
            .await
            .map_err(|err| err.into_response())?;

        let decode_access_token_service = app.resolver.decode_access_token_service();

        let raw_jwt = RawJwtAccessToken(bearer.token().to_string());
        let jwt_access_token = decode_access_token_service
            .execute(DecodeAccessToken { raw_jwt })
            .await
            .map_err(|err| err.into_response())?;

        Ok(ExtractJwtAccessToken(jwt_access_token))
    }
}

pub struct ExtractJwtRefreshToken(pub JwtRefreshToken);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractJwtRefreshToken
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| err.into_response())?;

        let Extension(app) = Extension::<App>::from_request_parts(parts, state)
            .await
            .map_err(|err| err.into_response())?;

        let decode_refresh_token_service = app.resolver.decode_refresh_token_service();

        let raw_jwt = RawJwtRefreshToken(bearer.token().to_string());
        let jwt_refresh_token = decode_refresh_token_service
            .execute(DecodeRefreshToken { raw_jwt })
            .await
            .map_err(|err| err.into_response())?;

        Ok(ExtractJwtRefreshToken(jwt_refresh_token))
    }
}
