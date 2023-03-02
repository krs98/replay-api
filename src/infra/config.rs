use std::time;

use chrono;
use dotenv::dotenv;
use url::Url;

use crate::modules::error::Error;

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_DATABASE_POOL_SIZE: &str = "DATABASE_POOL_SIZE";
const ENV_DATABASE_CONNECTION_LIFETIME: &str = "DATABASE_CONNECTION_LIFETIME";
const ENV_REDIS_URL: &str = "REDIS_URL";
const ENV_REDIS_POOL_SIZE: &str = "REDIS_POOL_SIZE";
const ENV_REDIS_CONNECTION_LIFETIME: &str = "REDIS_CONNECTION_LIFETIME";
const ENV_HTTP_PORT: &str = "PORT";
const ENV_JWT_ACCESS_TOKEN_SECRET: &str = "JWT_ACCESS_TOKEN_SECRET";
const ENV_JWT_REFRESH_TOKEN_SECRET: &str = "JWT_REFRESH_TOKEN_SECRET";
const ENV_JWT_ACCESS_TOKEN_DURATION: &str = "JWT_ACCESS_TOKEN_DURATION";
const ENV_JWT_REFRESH_TOKEN_DURATION: &str = "JWT_REFRESH_TOKEN_DURATION";

const POSTGRES_SCHEME: &str = "postgresql";
const REDIS_SCHEME: &str = "redis";

#[derive(Debug, Clone)]
pub struct Config {
    pub db: Database,
    pub redis: Redis,
    pub http: Http,
    pub jwt: Jwt
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
    pub connection_lifetime: time::Duration
}

const DEFAULT_DATABASE_POOL_SIZE: u32 = 10;
const DEFAULT_DATABASE_CONNECTION_LIFETIME: u64 = 30 * 60; // 30m

#[derive(Debug, Clone)]
pub struct Redis {
    pub url: String,
    pub pool_size: u32,
    pub connection_lifetime: time::Duration
}

const DEFAULT_REDIS_POOL_SIZE: u32 = 10;
const DEFAULT_REDIS_CONNECTION_LIFETIME: u64 = 30 * 60; // 30m

#[derive(Debug, Clone)]
pub struct Http {
    pub port: u16
}

const DEFAULT_HTTP_PORT: u16 = 3000;

#[derive(Debug, Clone)]
pub struct Jwt {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_duration: chrono::Duration,
    pub refresh_token_duration: chrono::Duration
}

const DEFAULT_JWT_ACCESS_TOKEN_DURATION: i64 = 60 * 60; // 1 hour
const DEFAULT_JWT_REFRESH_TOKEN_DURATION: i64 = 1440 * 60; // 1 day

impl Config {
    pub fn load() -> Result<Config, Error> {
        dotenv().ok();

        let db = Database::load()?;
        let redis = Redis::load()?;
        let http = Http::load()?;
        let jwt = Jwt::load()?;
        let config = Config { db, redis, http, jwt };
        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), Error> {
        self.db.validate()?;
        self.redis.validate()?;

        Ok(())
    }
}

impl Database {
    fn load() -> Result<Database, Error> {
        let url = std::env::var(ENV_DATABASE_URL).map_err(|_| {
            env_not_found(ENV_DATABASE_URL)
        })?;

        let pool_size = std::env::var(ENV_DATABASE_POOL_SIZE).map_or(
            Ok(DEFAULT_DATABASE_POOL_SIZE), 
            |pool_size_str| pool_size_str.parse::<u32>()
        )?;

        let connection_lifetime = std::env::var(ENV_DATABASE_CONNECTION_LIFETIME).map_or(
            Ok(DEFAULT_DATABASE_CONNECTION_LIFETIME),
            |connection_lifetime_str| connection_lifetime_str.parse::<u64>()
        ).map(time::Duration::from_secs)?;

        let db = Database { url, pool_size, connection_lifetime };
        Ok(db)
    }

    fn validate(&self) -> Result<(), Error> {
        let url = Url::parse(&self.url)?;
        if url.scheme() == POSTGRES_SCHEME {
            Ok(())
        } else {
            Err(Error::InvalidArgument(String::from(
                "config: DATABASE_URL is not a valid Postgres url"
            )))
        }
    }
}

impl Redis {
    fn load() -> Result<Redis, Error> {
        let url = std::env::var(ENV_REDIS_URL).map_err(|_| {
            env_not_found(ENV_REDIS_URL)
        })?;

        let pool_size = std::env::var(ENV_REDIS_POOL_SIZE).map_or(
            Ok(DEFAULT_REDIS_POOL_SIZE), 
            |pool_size_str| pool_size_str.parse::<u32>()
        )?;

        let connection_lifetime = std::env::var(ENV_REDIS_CONNECTION_LIFETIME).map_or(
            Ok(DEFAULT_REDIS_CONNECTION_LIFETIME),
            |connection_lifetime_str| connection_lifetime_str.parse::<u64>()
        ).map(time::Duration::from_secs)?;

        let redis = Redis { url, pool_size, connection_lifetime };
        Ok(redis)
    }

    fn validate(&self) -> Result<(), Error> {
        let url = Url::parse(&self.url)?;
        if url.scheme() == REDIS_SCHEME {
            Ok(())
        } else {
            Err(Error::InvalidArgument(String::from(
                "config: DATABASE_URL is not a valid Postgres url"
            )))
        }
    }
}

impl Http {
    fn load() -> Result<Http, Error> {
        let port = std::env::var(ENV_HTTP_PORT).map_or(
            Ok(DEFAULT_HTTP_PORT),
            |port_str| port_str.parse::<u16>()
        )?;

        let http = Http { port };
        Ok(http)
    }
}

impl Jwt {
    fn load() -> Result<Jwt, Error> {
        let access_token_secret = std::env::var(ENV_JWT_ACCESS_TOKEN_SECRET).map_err(|_| {
            env_not_found(ENV_JWT_ACCESS_TOKEN_SECRET)
        })?;

        let refresh_token_secret = std::env::var(ENV_JWT_REFRESH_TOKEN_SECRET).map_err(|_| {
            env_not_found(ENV_JWT_REFRESH_TOKEN_SECRET)
        })?;

        let access_token_duration = std::env::var(ENV_JWT_ACCESS_TOKEN_DURATION).map_or(
            Ok(DEFAULT_JWT_ACCESS_TOKEN_DURATION),
            |token_duration_str| token_duration_str.parse::<i64>()
        ).map(chrono::Duration::minutes)?;

        let refresh_token_duration = std::env::var(ENV_JWT_REFRESH_TOKEN_DURATION).map_or(
            Ok(DEFAULT_JWT_REFRESH_TOKEN_DURATION),
            |refresh_token_duration_str| refresh_token_duration_str.parse::<i64>()
        ).map(chrono::Duration::minutes)?;

        let jwt = Jwt { 
            access_token_secret, 
            refresh_token_secret,
            access_token_duration, 
            refresh_token_duration 
        };
        Ok(jwt)
    }
}

fn env_not_found(var: &str) -> Error {
    Error::NotFound(format!("config: {var} env var not found"))
}
