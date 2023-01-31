use crate::http::{ApiContext, Result};
use anyhow::Context;
use axum::extract::{Extension, Query};
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::builder::Str;
use time::{PrimitiveDateTime, Time};

use crate::http::error::{Error, ResultExt};

pub fn router() -> Router {
    Router::new()
        .route("/api/parameters", get(get_parameters))
        .route("/api/submit", post(submit_date))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TargetBody<T> {
    data: T,
}

#[derive(serde::Deserialize)]
struct NewTarget {
    temperature: f64,
    humidity: f64,
}

#[derive(serde::Deserialize,serde::Serialize)]
struct DateTarget {
    data: f64,
    date: PrimitiveDateTime,
}

#[derive(serde::Deserialize)]
pub struct DataQuery {
    data_type: String,
    start_date: PrimitiveDateTime,
    end_date: PrimitiveDateTime
}

#[axum::debug_handler]
async fn get_parameters(
    ctx: Extension<ApiContext>,
    query: Query<DataQuery>,
) -> Result<Json<TargetBody<Vec<DateTarget>>>> {
    let res = sqlx::query_as!(
            DateTarget,
        "select temperature as data, created_at as date  from temperature where created_at >= ? and created_at <= ?",
        query.start_date,
        query.end_date
    ).fetch_all(&ctx.db).await?;
    Ok(Json(TargetBody {
        data: res
    }))
}


async fn submit_date(
    ctx: Extension<ApiContext>,
    Json(req): Json<NewTarget>,
) -> Result<Json<TargetBody<bool>>> {
    let res = sqlx::query!(
        "insert into temperature ( temperature, humidity) values (?, ?) ",
        req.temperature,
        req.humidity,
    ).execute(&ctx.db).await?;

    Ok(Json(TargetBody::<bool> {
        data: true,
    }))
}
