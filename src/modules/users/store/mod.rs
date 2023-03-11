use crate::modules::error::Error;

use super::{User, LoginConnection, Username, Email};
use async_trait::async_trait;
use auto_impl::auto_impl;
use sqlx::{PgPool, PgExecutor, QueryBuilder};

#[async_trait]
#[auto_impl(&, Arc)]
pub trait UserStore {
    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, Error>;
    async fn save(&self, user: &User) -> Result<(), Error>;
}

#[derive(Debug)]
pub(in crate::modules::users) struct PgUserStore {
    pub pool: PgPool,
}

impl PgUserStore {
    pub(in crate::modules::users) fn new(pool: PgPool) -> Self {
        PgUserStore { pool }
    }
}

#[async_trait]
impl UserStore for PgUserStore {
    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
                select 
                    u.id,
                    u.username as "username: Username", 
                    u.email as "email: Email", 
                    u.created_at, 
                    array_agg((
                        lg.id,
                        lg.provider,
                        lg.access_token,
                        lg.created_at,
                        lg.last_connection
                    )) as "login_connections!: Vec<LoginConnection>"
                from users as u
                left outer join login_connections as lg
                on u.id = lg.user_id
                where username = $1
                group by u.id
            "#,
            username.as_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    async fn save(&self, user: &User) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        save_user_root(&mut transaction, user).await?; 
        save_login_connections(&mut transaction, user, &user.login_connections).await?;
        transaction.commit().await?;

        Ok(())
    }

}

async fn save_user_root<'c, E: PgExecutor<'c>>(executor: E, user: &User) -> Result<(), Error> {
    sqlx::query!(
        r#"
            insert into users (id, username, email, created_at) values ($1, $2, $3, $4)
            on conflict (id) do 
            update set username = excluded.username, 
                       email = excluded.email
        "#,
        0,
        user.username.as_inner(),
        user.email.as_ref().map(|email| email.as_inner()),
        user.created_at
    )
    .execute(executor)
    .await
    .map(|_| ())
    .map_err(|err| {
        tracing::error!("{}", err.to_string());
        err.into()
    })
}

async fn save_login_connections<'c, E: PgExecutor<'c>>(
    executor: E, 
    user: &User,
    login_connections: &Vec<LoginConnection>
) -> Result<(), Error> {
    let mut builder = QueryBuilder::new(
        "insert into login_connections (id, user_id, provider, access_token, created_at, last_connection) "
    );

    let query = builder.push_values(login_connections, |mut builder, connection| {
        builder.push_bind(connection.id)
            .push_bind(user.id)
            .push_bind(&connection.provider)
            .push_bind(&connection.access_token)
            .push_bind(connection.created_at)
            .push_bind(connection.last_connection);

    })
    .push(r#" 
        on conflict (id) 
        do update set access_token = excluded.access_token,
                      last_connection = excluded.last_connection
    "#)
    .build();

    query.execute(executor)
        .await
        .map(|_| ())
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            err.into()
        })
}
