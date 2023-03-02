use axum::{Server, Router, Extension};
use ::tracing::{info, error};

use crate::{api, modules::error::Error, infra::tracing};

use super::{config::Config, App, db, redis};

pub async fn run() -> Result<(), Error> {
    let config = Config::load()?;

    tracing::init();

    let router = api::router();
    let pg_pool = db::connect(config.db).await?;
    let redis_pool = redis::connect(config.redis)?;
    let app = App::new(config.jwt, pg_pool, redis_pool);

    let routes = Router::new()
        .nest("/api", router)
        .layer(Extension(app));

    // TODO: use `try_from` instead of `from`
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], config.http.port)); 
    let app = routes.into_make_service();

    let result = Server::bind(&addr).serve(app).await;
    // These two traces don't work. It seems to be related to the start
    // of the server.
    match result {
        Ok(_) => info!("Server started at port {}", addr),
        Err(_) => error!("Couldn't start the server.")
    }

    Ok(())
}
