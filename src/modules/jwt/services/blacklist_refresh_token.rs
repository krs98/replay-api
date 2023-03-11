use crate::{
    infra::{Resolver, Service, ServiceArgs},
    modules::jwt::{JwtRefreshToken, JwtStore},
    Error,
};

pub struct BlacklistRefreshToken {
    pub refresh_token: JwtRefreshToken,
}

impl ServiceArgs for BlacklistRefreshToken {
    type Output = Result<(), Error>;
}

async fn execute(
    BlacklistRefreshToken { refresh_token }: BlacklistRefreshToken,
    jwt_store: impl JwtStore,
) -> Result<(), Error> {
    jwt_store.blacklist_token(refresh_token).await
}

impl Resolver {
    pub fn blacklist_refresh_token_service(&self) -> impl Service<BlacklistRefreshToken> {
        self.service(|resolver, service: BlacklistRefreshToken| async move {
            let jwt_store = resolver.jwt_store();
            execute(service, jwt_store).await
        })
    }
}
