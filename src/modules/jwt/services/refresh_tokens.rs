use crate::{modules::jwt::{JwtRefreshToken, JwtAccessToken, JwtStore, AccessTokenSubject, RefreshTokenSubject}, Error, infra::{ServiceArgs, Service, Resolver}};

use super::EncodeTokens;

pub struct RefreshTokens {
    pub refresh_token: JwtRefreshToken
}

impl ServiceArgs for RefreshTokens {
    type Output = Result<(JwtAccessToken, JwtRefreshToken), Error>;
}

async fn execute(
    RefreshTokens { refresh_token }: RefreshTokens,
    encode_tokens_service: impl Service<EncodeTokens>,
    jwt_store: impl JwtStore
) -> Result<(JwtAccessToken, JwtRefreshToken), Error> {
    jwt_store.blacklist_token(refresh_token.clone()).await?;

    let username = refresh_token.claims.sub.into_inner();
    encode_tokens_service.execute(EncodeTokens { 
        access_token_subject: AccessTokenSubject(username.clone()), 
        refresh_token_subject: RefreshTokenSubject(username) 
    }).await
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
