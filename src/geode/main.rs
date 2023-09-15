use crystal::queue::connect_to_queue;
use crystal::request::Client;
use crystal::task::ScalpingTask;
use crystal::db::initialize_db_manager;

use dotenvy::dotenv;
use fang::asynk::async_queue::AsyncQueueable;
use fang::AsyncRunnable;
use std::env;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    direct: bool,

    url: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let cli = Cli::parse();

    let database_url = env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set");

    log::info!("Starting...");

    // Initialize DB Pool for crystal operations
    initialize_db_manager(database_url.clone()).await;

    let event_id = cli.url.split("/").last().unwrap();

    // Run locally?
    if cli.direct {
        crystal::scalp::scalp(event_id.to_string(), vec!["1".to_owned()])
            .await
            .unwrap();
        return;
    }

    // Connect & create pool to task queue
    let mut queue = connect_to_queue(database_url).await;
    log::info!("Queue connected...");

    // Fetch event details
    let client = Client::new();
    let sale_client = client.product(event_id.to_string()).await.unwrap();

    // Queue new task for workers
    let task = ScalpingTask::new(
        event_id.to_string(),
        vec!["1".to_owned()],
        sale_client.sale.product.date_sales_from,
    );

    queue
        .schedule_task(&task as &dyn AsyncRunnable)
        .await
        .unwrap();
}
