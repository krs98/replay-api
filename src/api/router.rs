use axum::{
    routing::{get, post},
    Router,
};

use super::routes::{
    auth,
    users
};

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth())
        .nest("/users", users())
}

fn auth() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
}

fn users() -> Router {
    Router::new()
        .route("/me", get(users::me))
}
