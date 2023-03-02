use sqlx::{postgres::PgPoolOptions, PgPool};

use super::config;

pub async fn connect(config: config::Database) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.pool_size)
        .max_lifetime(config.connection_lifetime)
        .connect(&config.url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
