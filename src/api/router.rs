use axum::{routing::{get, post, put}, Router};

use super::routes::auth;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth())
}

fn auth() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/me", get(auth::me))
        .route("/register", post(auth::register))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
}
