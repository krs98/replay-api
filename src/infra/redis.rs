use crate::Error;

use super::config;

pub type RedisPool = r2d2::Pool<redis::Client>;

pub fn connect(config: config::Redis) -> Result<r2d2::Pool<redis::Client>, Error> {
    let redis_client =
        redis::Client::open(config.url).map_err(std::convert::Into::<Error>::into)?;

    r2d2::Pool::builder()
        .max_size(config.pool_size)
        .max_lifetime(Option::Some(config.connection_lifetime))
        .build(redis_client)
        .map_err(|err| err.into())
}
