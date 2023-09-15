use fang::asynk::async_queue::AsyncQueue;
use postgres_native_tls::MakeTlsConnector;
use crate::db::create_db_connector;

pub async fn connect_to_queue(database_url: String) -> AsyncQueue<MakeTlsConnector> {
    let max_pool_size: u32 = 3;
    let mut queue = AsyncQueue::builder()
        .uri(database_url)
        .max_pool_size(max_pool_size)
        .build();
    let connector = create_db_connector();
    queue.connect(connector).await.unwrap();

    queue
}
