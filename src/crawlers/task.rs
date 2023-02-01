use std::time::Duration;

use tracing::info;

const SLEEP_TIME: u64 = 1000 * 60 * 30;

pub fn task() {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(SLEEP_TIME)).await;
        }
    });
}
