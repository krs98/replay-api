use crate::{
    infra::{Resolver, Service, ServiceArgs},
    modules::jwt::{
        AccessTokenSubject, JwtAccessToken, JwtRefreshToken, JwtStore, RefreshTokenSubject,
    },
    Error,
};

use super::{EncodeTokens, EncodeTokensOutput};

pub struct RefreshTokens {
    pub refresh_token: JwtRefreshToken,
}

pub struct RefreshTokensOutput {
    pub access_token: JwtAccessToken,
    pub refresh_token: JwtRefreshToken
}

impl std::convert::From<EncodeTokensOutput> for RefreshTokensOutput {
    fn from(value: EncodeTokensOutput) -> Self {
        RefreshTokensOutput { 
            access_token: value.access_token, 
            refresh_token: value.refresh_token
        }
    }
}

impl ServiceArgs for RefreshTokens {
    type Output = Result<RefreshTokensOutput, Error>;
}

async fn execute(
    RefreshTokens { refresh_token }: RefreshTokens,
    encode_tokens_service: impl Service<EncodeTokens>,
    jwt_store: impl JwtStore,
) -> Result<RefreshTokensOutput, Error> {
    jwt_store.blacklist_token(refresh_token.clone()).await?;

    let username = refresh_token.claims.sub.into_inner();

    let encode_tokens_input = EncodeTokens {
        access_token_subject: AccessTokenSubject(username.clone()),
        refresh_token_subject: RefreshTokenSubject(username),
    };
    encode_tokens_service.execute(encode_tokens_input)
        .await
        .map(std::convert::Into::<RefreshTokensOutput>::into)
}

impl Resolver {
    pub fn refresh_tokens_service(&self) -> impl Service<RefreshTokens> {
        self.service(|resolver, service: RefreshTokens| async move {
            let encode_tokens_service = resolver.encode_tokens_service();
            let jwt_store = resolver.jwt_store();

            execute(service, encode_tokens_service, jwt_store).await
        })
    }
}
