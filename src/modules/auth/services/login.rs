use crate::{
    infra::{Service, ServiceArgs, Resolver}, 
    modules::{
        users::{model::{Username, Password}, UserStore}, 
        error::{Error, AppError}, jwt::{AccessTokenSubject, JwtAccessToken, JwtRefreshToken, EncodeTokens, RefreshTokenSubject}
    }
};

pub struct Login {
    pub username: String,
    pub password: String
}

impl ServiceArgs for Login {
    type Output = Result<(JwtAccessToken, JwtRefreshToken), Error>;
}

async fn execute(
    Login { username, password }: Login,
    encode_tokens_service: impl Service<EncodeTokens>,
    user_store: impl UserStore
) -> Result<(JwtAccessToken, JwtRefreshToken), Error> {
    let username = Username::try_from(username)?;
    let password = Password::try_from(password)?;

    let user = user_store.find_by_username(username.clone()).await?;
    let Some(user) = user else {
        return Err(AppError::UserNotFound.into())
    };

    if user.password != password {
        return Err(AppError::InvalidPassword.into())
    }

    encode_tokens_service.execute(EncodeTokens { 
        access_token_subject: AccessTokenSubject(user.username.clone()), 
        refresh_token_subject: RefreshTokenSubject(user.username)
    }).await
}

impl Resolver {
    pub fn login_service(&self) -> impl Service<Login> {
        self.service(|resolver, service: Login| async move {
            let user_store = resolver.user_store();
            let encode_tokens_service = resolver.encode_tokens_service();

            execute(service, encode_tokens_service, user_store).await
        })
    }
}
