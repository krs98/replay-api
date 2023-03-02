use chrono::Utc;

use crate::{
    infra::{Service, ServiceArgs, Resolver}, 
    modules::{
        users::{model::{Email, Password, Username}, UserStore, User}, 
        error::{Error, AppError}, jwt::{JwtAccessToken, JwtRefreshToken, AccessTokenSubject, EncodeTokens, RefreshTokenSubject}
    }
};

#[derive(Debug)]
pub struct Register {
    pub username: String,
    pub email: String,
    pub password: String
}

impl ServiceArgs for Register {
    type Output = Result<(JwtAccessToken, JwtRefreshToken), Error>;
}

async fn execute(
    register: Register,
    encode_tokens_service: impl Service<EncodeTokens>,
    user_store: impl UserStore
) -> Result<(JwtAccessToken, JwtRefreshToken), Error> {
    let username = Username::try_from(register.username.clone())?;
    let email = Email::try_from(register.email.clone())?;
    let password = Password::try_from(register.password.clone())?;

    let user = user_store.find_by_username(username.clone()).await?;
    if user.is_some() {
        return Err(AppError::UserAlreadyExists.into())
    }

    let user = User { 
        username: username.clone(), 
        email, 
        password,
        created_at: Utc::now()
    };

    user_store.save(user).await?;

    encode_tokens_service.execute(EncodeTokens { 
        access_token_subject: AccessTokenSubject(username.clone()), 
        refresh_token_subject: RefreshTokenSubject(username) 
    }).await
}

impl Resolver {
    pub fn register_service(&self) -> impl Service<Register> {
        self.service(|resolver, service: Register| async move {
            let user_store = resolver.user_store();
            let encode_access_tokens = resolver.encode_tokens_service();

            execute(service, encode_access_tokens, user_store).await
        })
    }
}
