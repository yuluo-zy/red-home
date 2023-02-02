use std::time::Duration;
use anyhow::Context;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

use tracing::info;
use crate::crawlers::Spider;
use crate::http::spider_data::{DateTarget, Water};
use crate::http::CRUDData;

const SLEEP_TIME: u64 = 1000 * 60 * 1;
const ELECTRICITY_URL: &str = "http://www.wap.cnyiot.com/nat/nat.aspx?id=ofQeSQvSx4TKbA3zLZDW9g%3D%3D&by=a";
const WATER_FEE_URL: &str = "http://www.wap.cnyiot.com/nat/nat.aspx?id=ZTJNRu9%2F3vqBO0Rq7RoQ6w%3D%3D&by=a";

#[derive(Clone, Default, Debug)]
pub struct WaterFee {
    remainder: f64,
    unit: f64,
}

impl Spider for WaterFee {
    fn name(&self) -> String {
        "水费".to_string()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Electricity {
    remainder: f64,
    unit: f64,
}

impl Spider for Electricity {
    fn name(&self) -> String {
        "电费".to_string()
    }
}

impl From<Electricity> for DateTarget {
    fn from(item: Electricity) -> Self {
        Self{
            remainder: item.remainder,
            unit: item.unit,
            created_at: None,
        }
    }
}

impl From<WaterFee> for Water {
    fn from(item: WaterFee) -> Self {
        Self{
            remainder: item.remainder,
            unit: item.unit,
            created_at: None,
        }
    }
}



pub fn task(db_url: String) {
    tokio::spawn(async move {
        let db = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .context("could not connect to database_url").unwrap();
        loop {
            let mut electricity = Electricity::default();
            let html = electricity.get_html(ELECTRICITY_URL).await.unwrap();
            let (c1, c2) = electricity.scrape(html).unwrap();
            electricity.remainder = c1;
            electricity.unit = c2;
            info!("electricity :: {:?}", &electricity);
            let electricity_save: DateTarget = electricity.into();

            electricity_save.save(&db).await.unwrap();

            let mut water = WaterFee::default();
            let html = water.get_html(WATER_FEE_URL).await.unwrap();
            let (c1, c2) = water.scrape(html).unwrap();
            water.remainder = c1;
            water.unit = c2;
            info!("electricity :: {:?}", &water);
            let water_save: Water = water.into();
            water_save.save(&db).await.unwrap();

            tokio::time::sleep(Duration::from_millis(SLEEP_TIME)).await;
        }
    });
}
