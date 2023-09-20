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
use juniper::GraphQLObject;
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
    pub options: TaskOptions,
}

impl ScalpingTask {
    pub fn new(
        event_id: String,
        account_ids: AccountIDList,
        sale_start: DateTime<Utc>,
        options: TaskOptions,
    ) -> Self {
        Self {
            event_id,
            account_ids,
            sale_start,
            options,
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

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[serde(crate = "fang::serde")]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Options for a task")]
pub struct TaskOptions {
    pub target_price: Option<i32>,
    pub target_name: Option<String>,
    pub use_regex: bool,
    pub ignore_membership: bool,
}

impl Default for TaskOptions {
    fn default() -> Self {
        Self {
            target_price: None,
            target_name: None,
            use_regex: false,
            ignore_membership: true,
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
            self.options.clone(),
        )
        .await?;

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
        5
    }

    fn task_type(&self) -> String {
        "common".to_string()
    }
}
