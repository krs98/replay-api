use crate::{Error, infra::{ServiceArgs, Resolver, Service, id::IdProvider}, modules::{auth::{store::{AsOAuthStore, OAuthUserResponse}, OAuthProvider, OAuthCode}, users::{UserStore, User, LoginConnection}, jwt::{EncodeTokens, JwtAccessToken, JwtRefreshToken, AccessTokenSubject, RefreshTokenSubject, EncodeTokensOutput}}};

#[derive(Debug, Clone)]
pub struct Login {
    pub provider: OAuthProvider,
    pub code: OAuthCode
}

pub struct LoginOutput {
    pub access_token: JwtAccessToken,
    pub refresh_token: JwtRefreshToken
}

impl std::convert::From<EncodeTokensOutput> for LoginOutput {
    fn from(output: EncodeTokensOutput) -> Self {
        LoginOutput {
            access_token: output.access_token,
            refresh_token: output.refresh_token
        } 
    }
}

impl ServiceArgs for Login {
    type Output = Result<LoginOutput, Error>;
}

// A user can login through a third party oauth provider (Github, Gitlab or Bitbucket).
// After the authentication has succeeded, we create a new user in our database
// if it does not exist, otherwise we just create or update its corresponding login
// connection.
async fn execute(
    login: Login,
    oauth_proxy: impl AsOAuthStore,
    user_store: impl UserStore,
    encode_tokens_service: impl Service<EncodeTokens>,
    mut user_id_provider: impl IdProvider<User>,
    mut login_connection_id_provider: impl IdProvider<LoginConnection>,
) -> Result<LoginOutput, Error> {
    let oauth_store = oauth_proxy.as_oauth_store(&login.provider);

    let login_response = oauth_store.login(login.code).await?;
    let user_response = oauth_store.user(login_response.access_token.clone()).await?;
    
    let login_connection = LoginConnection::new(
        login_connection_id_provider.get(),
        login.provider, 
        login_response.access_token.clone()
    );
    let existing_user = user_store.find_by_username(&user_response.username).await?;

    let user = match existing_user {
        Some(mut user) => {
            user.add_or_update_login_connection(login_connection)?;
            user
        }
        None => {
            let id = user_id_provider.get();
            let OAuthUserResponse { username, email } = user_response;

            User::new(*id.as_inner(), username, email, login_connection)
        }
    };

    user_store.save(&user).await?;

    let encode_tokens_inputs = EncodeTokens {
        access_token_subject: AccessTokenSubject(user.username.clone()),
        refresh_token_subject: RefreshTokenSubject(user.username)
    };
    let tokens = encode_tokens_service.execute(encode_tokens_inputs).await?;

    Ok(tokens.into())
}

impl Resolver {
    pub fn login_with_github_service(&self) -> impl Service<Login> {
        self.service(|resolver, service: Login| async move {
            let oauth_proxy = resolver.oauth_proxy();
            let user_store = resolver.user_store();
            let encode_tokens_service = resolver.encode_tokens_service();
            let user_id_provider = resolver.user_id();
            let login_connection_id_provider = resolver.login_connection_id();

            execute(
                service, 
                oauth_proxy, 
                user_store,
                encode_tokens_service,
                user_id_provider,
                login_connection_id_provider
            ).await
        })
    }
}
