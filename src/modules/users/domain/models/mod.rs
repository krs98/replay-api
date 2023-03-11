use chrono::{DateTime, Utc};

use crate::{modules::{auth::OAuthProvider, error::AppError}, infra::{id::{IdProvider, SnowflakeIdProvider, Id}, Resolver}};

use super::{Username, Email, OAuthAccessToken};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Id<User>,
    pub username: Username,
    pub email: Option<Email>,
    pub created_at: DateTime<Utc>,
    pub login_connections: Vec<LoginConnection>,
}

#[derive(Debug, sqlx::FromRow, sqlx::Type)]
pub struct LoginConnection {
    pub id: Id<LoginConnection>,
    pub provider: OAuthProvider,
    pub access_token: OAuthAccessToken,
    pub created_at: DateTime<Utc>,
    pub last_connection: DateTime<Utc>,
}

impl User {
    pub fn new(
        id: Id<User>,
        username: Username,
        email: Option<Email>,
        login_connection: LoginConnection 
    ) -> Self {
        User {
            id,
            username,
            email,
            created_at: Utc::now(),
            login_connections: vec![login_connection]
        }
    }

    pub fn add_or_update_login_connection(
        &mut self, 
        new_connection: LoginConnection
    ) -> Result<(), AppError> {
        let existing_connection = self.login_connections.iter_mut().find(|existing_connection| {
            existing_connection.provider == new_connection.provider
        });

        if let Some(mut existing_connection) = existing_connection {
            existing_connection.access_token = new_connection.access_token;
            existing_connection.last_connection = Utc::now();
        } else {
            self.login_connections.push(new_connection); 
        }
        
        Ok(())
    }
}

impl LoginConnection {
    pub fn new(
        id: Id<LoginConnection>,
        provider: OAuthProvider,
        access_token: OAuthAccessToken
    ) -> Self {
        LoginConnection { 
            id,
            provider, 
            access_token, 
            created_at: Utc::now(), 
            last_connection: Utc::now() 
        }
    }
}

impl Resolver {
    pub fn user_id(&self) -> impl IdProvider<User> {
        SnowflakeIdProvider::<User>::new()
    }
}

impl Resolver {
    pub fn login_connection_id(&self) -> impl IdProvider<LoginConnection> {
        SnowflakeIdProvider::<LoginConnection>::new()
    }
}
