// We prefer to keep `main.rs` and `lib.rs` separate as it makes it easier to add extra helper
// binaries later which share code with the main project. It could save you from a nontrivial
// refactoring effort in the future.
//
// Whether to make `main.rs` just a thin shim that awaits a `run()` function in `lib.rs`, or
// to put the application bootstrap logic here is an open question. Both approaches have their
// upsides and their downsides. Your input is welcome!

// use anyhow::Context;
// use clap::Parser;
// use sqlx::mysql::MySqlPoolOptions;
// use tracing::Level;
// use tracing_subscriber::fmt;
//
// use red_home::config::Config;
// use red_home::crawlers::init_task;
// use red_home::http;
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     dotenv::dotenv().ok();
//
//     fmt().with_max_level(Level::INFO).init();
//
//     let config = Config::parse();
//    let db = MySqlPoolOptions::new()
//         .max_connections(10)
//         .connect(&config.database_url)
//         .await
//         .context("could not connect to database_url")?;
//     init_task();
//
//     http::serve(config, db).await?;
//
//     Ok(())
// }
use reqwest;
#[tokio::main]
async fn main()  -> Result<(),reqwest::Error> {

    // HTML
    let url = "http://www.wap.cnyiot.com/nat/nat.aspx?id=ZTJNRu9%2F3vqBO0Rq7RoQ6w%3D%3D&by=a";
    let client = reqwest::Client::builder().build().unwrap();
    let resp = client.get(url)
        .send()
        .await?;
    println!("Response: {:?} {}", resp.version(), resp.status());
    println!("Headers: {:#?}\n", resp.headers());
    println!("Body:{:#?}",resp.text().await?);

    Ok(())
}
