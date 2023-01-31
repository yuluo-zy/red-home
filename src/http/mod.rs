use std::net::{SocketAddr};
use crate::config::Config;
use anyhow::Context;
use axum::{Extension, Router};
use sqlx::MySqlPool;
use std::sync::Arc;
use tower::ServiceBuilder;
pub use error::{Error, ResultExt};

mod error;
mod temperature;


pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::trace::TraceLayer;

const SERVICE_PORT: &str = "0.0.0.0:8080";

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
    temperature::router()
}
