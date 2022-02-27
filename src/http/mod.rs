use crate::config::Config;
use anyhow::Context;
use axum::{http::Method, AddExtensionLayer, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer, Origin};

use crate::error::{Error, ResultExt};

mod base;

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(AddExtensionLayer::new(ApiContext {
                config: Arc::new(config),
                db,
            }))
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
                    .allow_methods(vec![Method::GET, Method::POST])
                    .allow_headers(Any),
            ),
    );
    axum::Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    base::router()
}
