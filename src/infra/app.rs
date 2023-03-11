use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::sync::Arc;

use crate::modules::{
    auth::resolver::AuthResolver, jwt::resolver::JwtResolver, users::resolver::UsersResolver 
};

use super::{config, redis::RedisPool};

#[derive(Clone)]
pub struct App {
    pub resolver: Resolver,
}

impl App {
    pub fn new(
        github_oauth_config: config::GithubOAuth,
        jwt_config: config::Jwt, 
        pg_pool: PgPool, 
        redis_pool: RedisPool
    ) -> Self {
        App {
            resolver: Resolver {
                auth_resolver: AuthResolver::new(github_oauth_config),
                jwt_resolver: JwtResolver::new(jwt_config, redis_pool),
                users_resolver: UsersResolver::new(pg_pool),
            },
        }
    }
}

#[derive(Clone)]
pub struct Resolver {
    pub auth_resolver: AuthResolver,
    pub jwt_resolver: JwtResolver,
    pub users_resolver: UsersResolver,
}

impl Resolver {
    pub fn by_ref(&self) -> Self {
        Resolver {
            auth_resolver: self.auth_resolver.clone(),
            jwt_resolver: self.jwt_resolver.clone(),
            users_resolver: self.users_resolver.clone(),
        }
    }

    pub fn resolve<T: Clone>(&self, register: &Register<T>) -> T {
        (register.0)()
    }
}

#[derive(Clone)]
pub struct Register<T>(Arc<dyn Fn() -> T + Sync + Send>);

impl<T> Register<T> {
    pub fn once(e: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        let cell = OnceCell::new();
        Register(Arc::new(move || cell.get_or_init(|| e.clone()).clone()))
    }
}
