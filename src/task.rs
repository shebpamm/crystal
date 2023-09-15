use chrono::{DateTime, Utc};
use fang::async_trait;
use fang::asynk::async_queue::AsyncQueueable;
use fang::serde::{Deserialize, Serialize};
use fang::typetag;
use fang::AsyncRunnable;
use fang::FangError;
use fang::Scheduled;
use juniper::GraphQLObject;
use tokio::time::Duration;

use crate::scalp::scalp;

#[derive(Serialize, Deserialize, GraphQLObject)]
#[serde(crate = "fang::serde")]
#[graphql(description = "A scheduled reservation task for a specific event")]
#[serde(rename_all = "camelCase")]
pub struct ScalpingTask {
    pub event_id: String,
    pub account_ids: Vec<String>,
    pub sale_start: DateTime<Utc>,
}

impl ScalpingTask {
    pub fn new(event_id: String, account_ids: Vec<String>, sale_start: DateTime<Utc>) -> Self {
        Self {
            event_id,
            account_ids,
            sale_start,
        }
    }
}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for ScalpingTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        scalp(
            self.event_id.clone(),
            self.account_ids.clone(),
        ).await?;

        Ok(())
    }

    fn cron(&self) -> Option<Scheduled> {
        Some(Scheduled::ScheduleOnce(
            self.sale_start - Duration::from_secs(10),
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
