use crate::http::{ApiContext, CRUDData, Result};
use async_trait::async_trait;

use axum::extract::{Extension, Query};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

pub fn router() -> Router {
    Router::new()
        .route("/api/electricity", get(get_electricity))
        .route("/api/get_water", get(get_water))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TargetBody<T> {
    data: T,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DateTarget {
    pub(crate) remainder: f64,
    pub unit: f64,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize)]
pub struct DataQuery {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

#[async_trait]
impl CRUDData<DateTarget, DataQuery> for DateTarget {
    async fn save(self, conn: &MySqlPool) -> Result<()> {
        sqlx::query!(
            "insert into electricity (remainder, unit) values (?, ?) ",
            self.remainder,
            self.unit,
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn find(query: DataQuery, conn: &MySqlPool) -> Result<Vec<DateTarget>> {
        let res = sqlx::query_as!(
            DateTarget,
            "select `remainder`, `unit`, `created_at`   from electricity where created_at >= ? and created_at <= ?",
            query.start_date,
            query.end_date
        )
        .fetch_all(conn)
        .await?;
        Ok(res)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Water {
    pub remainder: f64,
    pub unit: f64,
    pub created_at: Option<DateTime<Utc>>,
}

#[async_trait]
impl CRUDData<Water, DataQuery> for Water {
    async fn save(self, conn: &MySqlPool) -> Result<()> {
        sqlx::query!(
            "insert into water (remainder, unit) values (?, ?) ",
            self.remainder,
            self.unit,
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn find(query: DataQuery, conn: &MySqlPool) -> Result<Vec<Water>> {
        let res = sqlx::query_as!(
            Water,
            "select `remainder`, `unit`, `created_at`   from water where created_at >= ? and created_at <= ?",
            query.start_date,
            query.end_date
        )
            .fetch_all(conn)
            .await?;
        Ok(res)
    }
}

async fn get_electricity(
    ctx: Extension<ApiContext>,
    query: Query<DataQuery>,
) -> Result<Json<TargetBody<Vec<DateTarget>>>> {
    let res = DateTarget::find(query.0, &ctx.db).await?;
    Ok(Json(TargetBody { data: res }))
}

async fn get_water(
    ctx: Extension<ApiContext>,
    query: Query<DataQuery>,
) -> Result<Json<TargetBody<Vec<Water>>>> {
    let res = Water::find(query.0, &ctx.db).await?;
    Ok(Json(TargetBody { data: res }))
}
