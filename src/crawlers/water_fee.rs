use crate::crawlers::Spider;

const url: &str = "http://www.wap.cnyiot.com/nat/nat.aspx?id=ZTJNRu9%2F3vqBO0Rq7RoQ6w%3D%3D&by=a";

#[derive(Clone, Default)]
pub struct WaterFee {
    remainder: f64,
    unit: u8,
}

impl Spider<WaterFee> for WaterFee {
    fn name(&self) -> String {
        "水费".to_string()
    }

    fn urls(&self) -> &str {
        url
    }

    fn scrape(&self, html: String) -> anyhow::Result<WaterFee> {
        todo!()
    }

    fn subsequent(&self) {
        todo!()
    }
}
