use tower_http::cors::{CorsLayer, Any};

pub fn allow() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
}
