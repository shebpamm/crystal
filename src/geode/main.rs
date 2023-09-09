use crystal::task::ScalpingTask;
use dotenvy::dotenv;
use fang::asynk::async_queue::AsyncQueue;
use fang::asynk::async_queue::AsyncQueueable;
use fang::AsyncRunnable;
use fang::NoTls;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    log::info!("Starting...");
    let max_pool_size: u32 = 3;
    let mut queue = AsyncQueue::builder()
        .uri(database_url)
        .max_pool_size(max_pool_size)
        .build();

    queue.connect(NoTls).await.unwrap();
    log::info!("Queue connected...");

    let token = env::var("KIDE_API_TOKEN").expect("KIDE_API_TOKEN must be set");

    let test_task = ScalpingTask::new(
        "22b2e772-5889-4b18-bae9-24a3d05bfe3f".to_string(),
        token,
        chrono::Utc::now() + chrono::Duration::seconds(5),
    );

    queue
        .insert_task(&test_task as &dyn AsyncRunnable)
        .await
        .unwrap();
}
