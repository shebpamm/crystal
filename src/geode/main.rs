use crystal::queue::connect_to_queue;
use crystal::task::ScalpingTask;
use crystal::request::Client;

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

    let event_id = cli.url.split("/").last().unwrap();
    let token = env::var("KIDE_API_TOKEN").expect("KIDE_API_TOKEN must be set");

    // Run locally?
    if cli.direct {
        crystal::scalp::scalp(event_id.to_string(), token)
            .await
            .unwrap();
        return;
    }

    // Connect to queue
    let mut queue = connect_to_queue(database_url).await;
    log::info!("Queue connected...");

    // Fetch event details
    let client = Client::new(token.clone());
    let sale_client = client.product(event_id.to_string()).await.unwrap();

    // Queue new task for workers
    let test_task = ScalpingTask::new(
        event_id.to_string(),
        token,
        sale_client.sale.product.date_sales_from,
    );

    queue
        .insert_task(&test_task as &dyn AsyncRunnable)
        .await
        .unwrap();
}
