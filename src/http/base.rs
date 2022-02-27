use axum::{extract::Extension, response::Json, routing::get, Router};
use serde_json::{json, Value};

use crate::http::ApiContext;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
}

async fn index(ctx: Extension<ApiContext>) -> Json<Value> {
    Json(json!({ "status": "success".to_string(), "version": ctx.config.version.clone() }))
}

async fn health_check() -> &'static str {
    "ok"
}
