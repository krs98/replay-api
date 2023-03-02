use crate::{
    modules::{users::{model::User, UserStore}, error::{Error, AppError}, jwt::AccessTokenSubject}, 
    infra::{ServiceArgs, Service, Resolver}
};

pub struct Me {
    pub subject: AccessTokenSubject
}

impl ServiceArgs for Me {
    type Output = Result<User, Error>;
}

async fn execute(
    Me { subject }: Me,
    user_store: impl UserStore
) -> Result<User, Error> {
    let user = user_store.find_by_username(subject.into_inner()).await?;
    match user {
        Some(user) => Ok(user),
        // User no longer available?
        None => Err(AppError::UserNotFound.into())
    }
}

impl Resolver {
    pub fn me_service(&self) -> impl Service<Me> {
        self.service(|resolver, service: Me| async move {
            let user_store = resolver.user_store();
            execute(service, user_store).await
        })
    }
}
