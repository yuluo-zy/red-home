use crate::config::Config;
use anyhow::Context;
use async_trait::async_trait;
use axum::{Extension, Router};
pub use error::{Error, ResultExt};
use sqlx::MySqlPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;

mod error;
mod spider_data;
mod temperature;

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;

const SERVICE_PORT: &str = "0.0.0.0:8088";

#[async_trait]
pub trait CRUDData<I, Q> {
    async fn save(data: I, conn: &MySqlPool) -> Result<()>;
    async fn find(query: Q, conn: &MySqlPool) -> Result<Vec<I>>;
}

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: MySqlPool,
}

pub async fn serve(config: Config, db: MySqlPool) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                db,
            }))
            .layer(TraceLayer::new_for_http()),
    );

    axum::Server::bind(&SERVICE_PORT.parse::<SocketAddr>()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    temperature::router().merge(spider_data::router())
}
