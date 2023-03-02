use async_trait::async_trait;
use auto_impl::auto_impl;
use redis::Commands;
use tracing::debug;

use crate::{Error, infra::redis::RedisPool};

use super::{RawJwtRefreshToken, JwtRefreshToken};

#[async_trait]
#[auto_impl(&, Arc)]
pub trait JwtStore {
    async fn blacklist_token(&self, jwt_token: JwtRefreshToken) -> Result<(), Error>;
    async fn is_blacklisted(&self, raw_token: RawJwtRefreshToken) -> Result<bool, Error>;
}

#[derive(Debug)]
pub(in crate::modules::jwt) struct RedisJwtStore {
    pub pool: RedisPool
}

impl RedisJwtStore {
    pub(in crate::modules::jwt) fn new(pool: RedisPool) -> Self {
        RedisJwtStore { pool }
    }
}

#[async_trait]
impl JwtStore for RedisJwtStore {
    async fn blacklist_token(&self, jwt_token: JwtRefreshToken) -> Result<(), Error> {
        let mut conn = self.pool.get()?;
        
        let key = jwt_token.clone().raw.0;
        let exp = jwt_token.claims.exp.try_into().unwrap();

        conn.set(key.clone(), "")?;
        conn.expire_at(key, exp)?;

        Ok(())
    }

    async fn is_blacklisted(&self, raw_token: RawJwtRefreshToken) -> Result<bool, Error> {
        let mut conn = self.pool
            .get()
            .map_err(<r2d2::Error as std::convert::Into<Error>>::into)?;

        let value: Option<()> = conn
            .get(raw_token.0)
            .map_err(<redis::RedisError as std::convert::Into<Error>>::into)?;

        debug!("Retrieved jwt from redis: {:?}", value);

        Ok(value.is_some())
    }
}
