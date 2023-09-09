use diesel::PgConnection;
use diesel::Connection;
use dotenvy::dotenv;
use crystal::task::ScalpingTask;
use fang::asynk::async_queue::AsyncQueue;
use fang::asynk::async_worker_pool::AsyncWorkerPool;
use fang::run_migrations_postgres;
use std::env;
use std::time::Duration;
use fang::asynk::AsyncRunnable;
use postgres_native_tls::MakeTlsConnector;
use native_tls::TlsConnector;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // Typetag is not able to detect the use of this if we don't artificially use it here.
    //
    // https://github.com/dtolnay/typetag/issues/35
    let _: Box<dyn AsyncRunnable> = Box::new(ScalpingTask::new("".to_owned(), "".to_owned(), chrono::Utc::now()));

    let database_url = env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set");

    let mut connection = PgConnection::establish(&database_url).unwrap();

    log::info!("Running migrations...");

    run_migrations_postgres(&mut connection).unwrap();

    log::info!("Migrations done...");

    drop(connection);

    log::info!("Starting...");

    let connector = TlsConnector::builder()
        .build()
        .expect("Failed to build TLS connector");

    let connector = MakeTlsConnector::new(connector);


    let max_pool_size: u32 = 3;
    let mut queue = AsyncQueue::builder()
        .uri(database_url)
        .max_pool_size(max_pool_size)
        .build();

    queue.connect(connector).await.unwrap();
    log::info!("Queue connected...");

    let mut pool: AsyncWorkerPool<AsyncQueue<MakeTlsConnector>> = AsyncWorkerPool::builder()
        .number_of_workers(10_u32)
        .queue(queue.clone())
        .build();

    log::info!("Pool created ...");

    pool.start().await;
    log::info!("Workers started ...");

    tokio::time::sleep(Duration::from_secs(100)).await;
}
