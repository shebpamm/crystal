use crate::account::AccountIDList;
use crate::scalp::scalp;

use chrono::{DateTime, Utc};
use fang::async_trait;
use fang::asynk::async_queue::AsyncQueueable;
use fang::serde::{Deserialize, Serialize};
use fang::typetag;
use fang::AsyncRunnable;
use fang::FangError;
use fang::Scheduled;
use tokio::time::Duration;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "fang::serde")]
#[serde(rename_all = "camelCase")]
pub struct ScalpingTask {
    // TODO: Change this to type Uuid, including in api.rs
    pub event_id: String,
    pub account_ids: Vec<Uuid>,
    pub sale_start: DateTime<Utc>,
}

impl ScalpingTask {
    pub fn new(event_id: String, account_ids: AccountIDList, sale_start: DateTime<Utc>) -> Self {
        Self {
            event_id,
            account_ids,
            sale_start,
        }
    }
}

impl<'a> TryFrom<&'a Row> for ScalpingTask {
    type Error = anyhow::Error;

    fn try_from(row: &'a Row) -> Result<Self, anyhow::Error> {
        let data = row.try_get("metadata")?;

        Ok(serde_json::from_value(data)?)
    }
}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for ScalpingTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        scalp(self.event_id.clone(), self.account_ids.clone()).await?;

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
