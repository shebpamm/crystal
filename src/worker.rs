use fang::asynk::async_queue::AsyncQueue;
use fang::asynk::async_worker_pool::AsyncWorkerPool;
use postgres_native_tls::MakeTlsConnector;

pub fn create_worker_pool(
    queue: AsyncQueue<MakeTlsConnector>,
) -> AsyncWorkerPool<AsyncQueue<MakeTlsConnector>> {
    AsyncWorkerPool::builder()
        .number_of_workers(10_u32)
        .queue(queue.clone())
        .build()
}
