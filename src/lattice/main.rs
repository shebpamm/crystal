use crystal::task::ScalpingTask;
use dotenvy::dotenv;
use fang::asynk::AsyncRunnable;
use std::env;

use crystal::db::do_migrations;
use crystal::queue::connect_to_queue;
use crystal::worker::create_worker_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    // Typetag is not able to detect the use of this if we don't artificially use it here.
    //
    // https://github.com/dtolnay/typetag/issues/35
    let _: Box<dyn AsyncRunnable> = Box::new(ScalpingTask::new(
        "".to_owned(),
        "".to_owned(),
        chrono::Utc::now(),
    ));

    let database_url = env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set");

    log::info!("Running migrations...");
    do_migrations(database_url.clone());

    log::info!("Connecting to database...");
    let queue = connect_to_queue(database_url).await;

    log::info!("Queue connected...");

    let mut pool = create_worker_pool(queue);

    log::info!("Pool created ...");

    pool.start().await;
    log::info!("Workers started ...");

    tokio::signal::ctrl_c().await.unwrap();
}
