use crate::modules::error::Error;

use super::model::{User, Password, Email, Username};
use async_trait::async_trait;
use auto_impl::auto_impl;
use sqlx::PgPool;

#[async_trait]
#[auto_impl(&, Arc)]
pub trait UserStore {
    async fn find_by_username(&self, username: Username) -> Result<Option<User>, Error>;
    async fn save(&self, user: User) -> Result<(), Error>;
}

#[derive(Debug)]
pub(in crate::modules::users) struct PgUserStore {
    pub pool: PgPool
}

impl PgUserStore {
    pub(in crate::modules::users) fn new(pool: PgPool) -> Self {
        PgUserStore { pool }
    }
}

#[async_trait]
impl UserStore for PgUserStore {
    async fn find_by_username(&self, username: Username) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
                select username as "username: Username", email as "email: Email", password as "password: Password", created_at 
                from users where username = $1
            "#,
            username.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            err.into()
        })
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        sqlx::query!(
            "insert into users (username, email, password, created_at) values ($1, $2, $3, $4)", 
            user.username.into_inner(),
            user.email.into_inner(),
            user.password.into_inner(),
            user.created_at
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            err.into()
        })
    }
}
