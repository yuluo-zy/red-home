use crate::crawlers::Spider;

const url: &str = "http://www.wap.cnyiot.com/nat/nat.aspx?id=ofQeSQvSx4TKbA3zLZDW9g%3D%3D&by=a";

#[derive(Clone, Default)]
pub struct Electricity {
    remainder: f64,
    unit: u8,
}

impl Spider<Electricity> for Electricity {
    fn name(&self) -> String {
        "电费".to_string()
    }

    fn urls(&self) -> &str {
        url
    }

    fn scrape(&self, html: String) -> anyhow::Result<Electricity> {
        todo!()
    }

    fn subsequent(&self) {
        todo!()
    }
}
