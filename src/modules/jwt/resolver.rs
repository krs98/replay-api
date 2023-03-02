use std::sync::Arc;

use crate::infra::{config, Resolver, Register, redis::RedisPool};

use super::{RedisJwtStore, store, JwtStore};

#[derive(Clone)]
pub struct JwtResolver {
    jwt_config: Register<config::Jwt>,
    jwt_store: Register<Arc<RedisJwtStore>>
}

impl JwtResolver {
    pub fn new(jwt_config: config::Jwt, pool: RedisPool) -> Self {
        JwtResolver { 
            jwt_config: Register::once(jwt_config),
            jwt_store: Register::once(Arc::new(store::RedisJwtStore::new(pool)))
        }
    }
}

impl Resolver {
    pub(in crate::modules) fn jwt_config(&self) -> config::Jwt {
        self.resolve(&self.jwt_resolver.jwt_config)
    }

    pub(in crate::modules) fn jwt_store(&self) -> impl JwtStore {
        self.resolve(&self.jwt_resolver.jwt_store)
    }
}
