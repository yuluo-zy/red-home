use std::collections::HashMap;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{Html, Selector};
use sqlx::MySqlPool;
use tracing::info;


mod task;

pub fn init_task(db_url: String) {
    task::task(db_url);
}

#[async_trait]
pub trait Spider {
    fn name(&self) -> String;

    async fn get_html(&self, url: &str) -> Result<String> {
        let html = reqwest::get(url).await?.text().await?;
        Ok(html)
    }

    fn scrape(&self, html: String) -> Result<(f64, f64)> {
        let fragment = Html::parse_fragment(&*html);
        let selector1 = Selector::parse("#body > div:nth-child(2) > div:nth-child(2) > label").unwrap();
        let selector2 = Selector::parse("#body > div:nth-child(2) > div:nth-child(3) > label").unwrap();
        let h1 = fragment.select(&selector1).next().unwrap();
        let remainder = h1.text().collect::<Vec<&str>>().join("").parse::<f64>()?;
        let h2 = fragment.select(&selector2).next().unwrap();
        let unit = h2.text().collect::<Vec<&str>>().join("").parse::<f64>()?;
        Ok((remainder, unit))
    }
}

