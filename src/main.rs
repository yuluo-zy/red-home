use anyhow::Context;
use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use tracing::Level;
use tracing_subscriber::fmt;

use red_home::config::Config;
use red_home::crawlers::init_task;
use red_home::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    fmt().with_max_level(Level::INFO).init();

    let config = Config::parse();
   let db = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;
    init_task(config.database_url.clone());

    http::serve(config, db).await?;

    Ok(())
}
