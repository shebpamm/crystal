use chrono::{DateTime, Utc};
use fang::async_trait;
use fang::asynk::async_queue::AsyncQueueable;
use fang::serde::{Deserialize, Serialize};
use fang::typetag;
use fang::AsyncRunnable;
use fang::FangError;
use fang::Scheduled;
use tokio::time::Duration;

use crate::request::Client;
use crate::strategy::Count;

#[derive(Serialize, Deserialize)]
#[serde(crate = "fang::serde")]
pub struct ScalpingTask {
    pub event_id: String,
    pub account_token: String,
    pub sale_start: DateTime<Utc>,
}

impl ScalpingTask {
    pub fn new(event_id: String, account_token: String, sale_start: DateTime<Utc>) -> Self {
        Self {
            event_id,
            account_token,
            sale_start,
        }
    }
}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for ScalpingTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        let client = Client::new(self.account_token.clone());
        let sale_client = client.product(self.event_id.clone()).await.unwrap();

        // Block until the sale starts.
        // If there's over 2 seconds left until the sale starts, sleep for 1 second and
        // recheck.
        // If there's less than 2 seconds left until the sale starts, sleep for only 0.1 seconds.
        if sale_client.sale.variants.len() == 0 {
            log::debug!("Waiting for sale to start...");
            loop {
                let now = Utc::now();
                let diff = self.sale_start - now;
                log::debug!("{} seconds until sale starts", diff.num_seconds());
                if diff.num_seconds() > 2 {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                } else {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }

                let sale_client = client.product(self.event_id.clone()).await.unwrap();
                if sale_client.sale.variants.len() > 0 {
                    break;
                }
            }
        }

        // Begin reserving tickets
        log::info!("Reserving all variants...");
        for i in 1..21 {
            let _ = sale_client.reserve_all(&Count { count: i }).await;
        }
        log::info!("Done");

        Ok(())
    }

    fn cron(&self) -> Option<Scheduled> {
        Some(Scheduled::ScheduleOnce(
            self.sale_start - Duration::from_secs(2),
        ))
    }

    fn uniq(&self) -> bool {
        true
    }

    fn max_retries(&self) -> i32 {
        0
    }

    fn task_type(&self) -> String {
        "common".to_string()
    }
}
