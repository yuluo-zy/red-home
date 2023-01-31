use crate::http::{ApiContext, Result};
use anyhow::Context;
use axum::extract::Extension;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::http::error::{Error, ResultExt};

pub fn router() -> Router {
    Router::new()
        // .route("/api/temperature", get())
        // .route("/api/humidity", get(login_user))
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


async fn submit_date(
    ctx: Extension<ApiContext>,
    Json(req): Json<TargetBody<NewTarget>>,
) -> Result<Json<TargetBody<bool>>> {

    // let user_id = sqlx::query_scalar!(
    //     // language=PostgreSQL
    //     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
    //     req.user.username,
    //     req.user.email,
    //     password_hash
    // )
    // .fetch_one(&ctx.db)
    // .await
    // .on_constraint("user_username_key", |_| {
    //     Error::unprocessable_entity([("username", "username taken")])
    // })
    // .on_constraint("user_email_key", |_| {
    //     Error::unprocessable_entity([("email", "email taken")])
    // })?;

    Ok(Json(TargetBody::<bool>{
        data: true,
    }))
}
