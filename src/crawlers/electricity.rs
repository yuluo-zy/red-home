use tracing::info;

pub fn task() {
    tokio::spawn(async move {
        info!("kkkk");
    });
}