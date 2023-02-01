use std::collections::HashMap;
use anyhow::Result;
mod electricity;
mod task;
mod water_fee;

pub fn init_task() {
    electricity::task();
}


pub trait Spider<I> {
    fn name(&self) -> String;
    fn urls(&self, url: String);
    fn scrape(&self, html: String) -> Result<HashMap<String, Vec<I>>>;
    fn subsequent(&self);
}
