use fang::asynk::async_worker_pool::AsyncWorkerPool;
use fang::SleepParams;
use std::time::Duration;
use crate::queue::Queue;

pub fn create_worker_pool(
    queue: Queue,
) -> AsyncWorkerPool<Queue> {
    let sleep_params = SleepParams::builder()
        .sleep_period(Duration::from_secs(1))
        .min_sleep_period(Duration::from_secs(1))
        .max_sleep_period(Duration::from_secs(5))
        .sleep_step(Duration::from_secs(1))
        .build();

    AsyncWorkerPool::builder()
        .number_of_workers(10_u32)
        .queue(queue.clone())
        .sleep_params(sleep_params)
        .build()
}
