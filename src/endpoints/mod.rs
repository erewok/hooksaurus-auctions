use crate::config::Config;
use anyhow::Context;
use axum::{extract::Extension, http::Method, Router};
use minijinja::{Environment, Source};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer, Origin};

use crate::error::{Error, ResultExt};

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
    let mut source = Source::new();
    source.load_from_path("templates", &["html"]).unwrap();
    env.set_source(source);

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
                    .allow_origin(Origin::exact("http://localhost:8000".parse().unwrap()))
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
