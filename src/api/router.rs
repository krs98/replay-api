use axum::{
    routing::{get, post},
    Router,
};

use super::routes::auth;

pub fn router() -> Router {
    Router::new().nest("/auth", auth())
}

fn auth() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
}
