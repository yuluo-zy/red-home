use std::collections::HashMap;
use anyhow::Result;
mod electricity;
mod task;
mod water_fee;

pub fn init_task() {
    task::task();
}


pub trait Spider<I> {
    fn name(&self) -> String;
    fn urls(&self) -> &str;
    fn scrape(&self, html: String) -> Result<I>;
    fn subsequent(&self);
}

