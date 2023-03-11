use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::modules::{error::Error, users::Username};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenSubject(pub Username);

impl AccessTokenSubject {
    pub fn into_inner(self) -> Username {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenSubject(pub Username);

impl RefreshTokenSubject {
    pub fn into_inner(self) -> Username {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: AccessTokenSubject,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: RefreshTokenSubject,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RawJwtAccessToken(pub String);

#[derive(Debug, Clone, Serialize)]
pub struct RawJwtRefreshToken(pub String);

#[derive(Debug, Clone, Serialize)]
pub struct JwtAccessToken {
    pub raw: RawJwtAccessToken,
    pub claims: AccessTokenClaims,
}

#[derive(Debug, Clone, Serialize)]
pub struct JwtRefreshToken {
    pub raw: RawJwtRefreshToken,
    pub claims: RefreshTokenClaims,
}

impl JwtAccessToken {
    pub fn encode(
        subject: AccessTokenSubject,
        duration: Duration,
        signature: String,
    ) -> Result<Self, Error> {
        let expiration = Utc::now()
            .checked_add_signed(duration)
            .ok_or(Error::Internal)?
            .timestamp();

        let claims = AccessTokenClaims {
            sub: subject,
            exp: expiration,
        };
        let raw_jwt = encode(&claims, signature)?;
        let jwt_access_token = JwtAccessToken {
            raw: RawJwtAccessToken(raw_jwt),
            claims,
        };

        Ok(jwt_access_token)
    }

    pub fn decode(raw_jwt: RawJwtAccessToken, signature: String) -> Result<Self, Error> {
        let claims = decode::<AccessTokenClaims>(raw_jwt.0.as_str(), signature)?;
        let jwt_access_token = JwtAccessToken {
            raw: raw_jwt,
            claims,
        };

        Ok(jwt_access_token)
    }
}

impl JwtRefreshToken {
    pub fn encode(
        subject: RefreshTokenSubject,
        duration: Duration,
        signature: String,
    ) -> Result<Self, Error> {
        let expiration = Utc::now()
            .checked_add_signed(duration)
            .ok_or(Error::Internal)?
            .timestamp();

        let claims = RefreshTokenClaims {
            sub: subject,
            exp: expiration,
        };
        let raw_jwt = encode(&claims, signature)?;
        let jwt_refresh_token = JwtRefreshToken {
            raw: RawJwtRefreshToken(raw_jwt),
            claims,
        };

        Ok(jwt_refresh_token)
    }

    pub fn decode(raw_jwt: RawJwtRefreshToken, signature: String) -> Result<Self, Error> {
        let claims = decode::<RefreshTokenClaims>(raw_jwt.0.as_str(), signature)?;
        let jwt_refresh_token = JwtRefreshToken {
            raw: raw_jwt,
            claims,
        };

        Ok(jwt_refresh_token)
    }
}

fn encode<C: Serialize>(claims: &C, signature: String) -> Result<String, Error> {
    let header = Header::new(Algorithm::HS512);
    let key = EncodingKey::from_secret(signature.as_bytes());
    let token = jsonwebtoken::encode(&header, claims, &key).map_err(|_| {
        tracing::error!("Could not encode jwt claims!");
        Error::Internal
    })?;

    Ok(token)
}

fn decode<C: for<'a> Deserialize<'a>>(raw_jwt: &str, signature: String) -> Result<C, Error> {
    let key = &DecodingKey::from_secret(signature.as_bytes());
    let validation = &Validation::new(Algorithm::HS512);

    jsonwebtoken::decode::<C>(raw_jwt, key, validation)
        .map(|token_data| token_data.claims)
        .map_err(|_| {
            tracing::error!("Could not decode jwt claims!");
            Error::Internal
        })
}
