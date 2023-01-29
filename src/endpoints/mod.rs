use crate::config::Config;
use crate::error::Error;
use anyhow::Context;
use axum::{
    extract::Extension,
    http::{HeaderValue, Method},
    Router,
};
use minijinja::{Environment, Source};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

mod admin;
mod base;

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
    template_env: Environment<'static>,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let mut env = Environment::new();
    env.set_source(Source::from_path("templates"));

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                db,
                template_env: env,
            }))
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
                    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(Any),
            ),
    );
    axum::Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    base::router().merge(admin::router())
}
