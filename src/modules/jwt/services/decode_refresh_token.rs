use crate::{infra::{ServiceArgs, config, Service, Resolver}, Error, modules::{jwt::{JwtRefreshToken, RawJwtRefreshToken, JwtStore}, error::AppError}};

pub struct DecodeRefreshToken {
    pub raw_jwt: RawJwtRefreshToken
}

impl ServiceArgs for DecodeRefreshToken {
    type Output = Result<JwtRefreshToken, Error>;
}

async fn execute(
    DecodeRefreshToken { raw_jwt }: DecodeRefreshToken,
    jwt_config: config::Jwt,
    jwt_store: impl JwtStore
) -> Result<JwtRefreshToken, Error> {
    let is_blacklisted = jwt_store.is_blacklisted(raw_jwt.clone()).await?;
    if is_blacklisted {
        return Err(AppError::RefreshTokenIsNoLongerValid.into())
    }

    let signature = jwt_config.refresh_token_secret;  
    JwtRefreshToken::decode(raw_jwt, signature)
}

impl Resolver {
    pub fn decode_refresh_token_service(&self) -> impl Service<DecodeRefreshToken> {
        self.service(|resolver, service: DecodeRefreshToken| async move {
            let jwt_config = resolver.jwt_config();
            let jwt_store = resolver.jwt_store();

            execute(service, jwt_config, jwt_store).await
        })
    }
}
