use axum::{http::StatusCode, response::{Response, IntoResponse}, Json};
use serde_json::{json, Value};

fn success(code: StatusCode, data: Value) -> Response {
    let json = json!({
        "success": true,
        "data": data
    });

    (code, Json(json)).into_response()
}

fn failure(code: StatusCode, error: Value) -> Response {
    let json = json!({
        "success": false,
        "error": error
    });

    (code, Json(json)).into_response()
}

pub fn ok(data: Value) -> Response {
    success(StatusCode::OK, data)
}

pub fn created(data: Value) -> Response {
    success(StatusCode::CREATED, data)
}

pub fn bad_request(error: Value) -> Response {
    failure(StatusCode::BAD_REQUEST, error)
}

pub fn not_found(error: Value) -> Response {
    failure(StatusCode::NOT_FOUND, error)
}

pub fn conflict(error: Value) -> Response {
    failure(StatusCode::CONFLICT, error)
}

pub fn internal_error(error: Value) -> Response {
    failure(StatusCode::INTERNAL_SERVER_ERROR, error)
}

