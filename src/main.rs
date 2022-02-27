use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use hooksaurus_auctions::config::Config;
use hooksaurus_auctions::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "hooksaurus_auctions=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    sqlx::migrate!().run(&db).await?;

    http::serve(config, db).await?;

    Ok(())
}
