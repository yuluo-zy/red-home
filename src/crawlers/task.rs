use std::time::Duration;

use tracing::info;

const SLEEP_TIME: u64 = 1000 * 60 * 30;

pub async fn task() {
    tokio::spawn(async move {
        loop {
            info!("kkkk");
            tokio::time::sleep(Duration::from_millis(SLEEP_TIME)).await;
        }
    });
}
