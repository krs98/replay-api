use crate::{infra::{ServiceArgs, config, Service, Resolver}, Error, modules::jwt::{RawJwtAccessToken, JwtAccessToken}};

pub struct DecodeAccessToken {
    pub raw_jwt: RawJwtAccessToken
}

impl ServiceArgs for DecodeAccessToken {
    type Output = Result<JwtAccessToken, Error>;
}

async fn execute(
    DecodeAccessToken { raw_jwt }: DecodeAccessToken,
    jwt_config: config::Jwt
) -> Result<JwtAccessToken, Error> {
    let signature = jwt_config.access_token_secret;  
    JwtAccessToken::decode(raw_jwt, signature)
}

impl Resolver {
    pub fn decode_access_token_service(&self) -> impl Service<DecodeAccessToken> {
        self.service(|resolver, service: DecodeAccessToken| async move {
            let jwt_config = resolver.jwt_config();
            execute(service, jwt_config).await
        })
    }
}
