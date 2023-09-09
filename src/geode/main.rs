use crystal::task::ScalpingTask;
use dotenvy::dotenv;
use fang::asynk::async_queue::AsyncQueue;
use fang::asynk::async_queue::AsyncQueueable;
use fang::AsyncRunnable;
use fang::NoTls;
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    log::info!("Starting...");
    let max_pool_size: u32 = 3;
    let mut queue = AsyncQueue::builder()
        .uri(database_url)
        .max_pool_size(max_pool_size)
        .build();

    queue.connect(NoTls).await.unwrap();
    log::info!("Queue connected...");

    let event_id = cli.url.split("/").last().unwrap();
    let token = env::var("KIDE_API_TOKEN").expect("KIDE_API_TOKEN must be set");

    // Run locally?
    if cli.direct {
        crystal::scalp::scalp(event_id.to_string(), token)
            .await
            .unwrap();
        return;
    }

    // Queue new task for workers
    let test_task = ScalpingTask::new(
        event_id.to_string(),
        token,
        chrono::Utc::now() + chrono::Duration::seconds(5),
    );

    queue
        .insert_task(&test_task as &dyn AsyncRunnable)
        .await
        .unwrap();
}
