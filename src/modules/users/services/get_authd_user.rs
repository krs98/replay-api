use crate::{modules::{jwt::{JwtAccessToken, AccessTokenSubject}, users::{User, UserStore}, error::AppError}, infra::{ServiceArgs, Resolver, Service}, Error};

pub struct GetAuthdUser {
    pub jwt: JwtAccessToken
}

pub struct GetAuthdUserOutput {
    pub user: User
}

impl ServiceArgs for GetAuthdUser {
    type Output = Result<GetAuthdUserOutput, Error>;
}

async fn execute(
    GetAuthdUser { jwt }: GetAuthdUser,
    user_store: impl UserStore
) -> Result<GetAuthdUserOutput, Error> {
    let AccessTokenSubject(username) = jwt.claims.sub;
    let Some(user) = user_store.find_by_username(&username).await? else {
        return Err(AppError::UserNotFound.into())
    };

    let output = GetAuthdUserOutput { user };
    Ok(output)

}

impl Resolver {
    pub fn get_authd_user_service(&self) -> impl Service<GetAuthdUser> {
        self.service(|resolver, service: GetAuthdUser| async move {
            let user_store = resolver.user_store();
            execute(service, user_store).await
        })
    }
}
