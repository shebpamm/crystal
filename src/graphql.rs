use chrono::{DateTime, Utc};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject};

use crate::account::{fetch_all_kide_accounts, fetch_kide_accounts, KideAccount};
use crate::db::get_db_manager;
use crate::queue::Queue;
use crate::task::ScalpingTask;

// Auxillary struct, that mirrors the ScalpingTask struct but instead provides KideAccounts and not account_ids
#[derive(GraphQLObject)]
#[graphql(description = "A task")]
struct Task {
    event_id: String,
    accounts: Vec<KideAccount>,
    sale_start: DateTime<Utc>,
    // options: TaskOptions,
}

#[derive(GraphQLObject)]
#[graphql(description = "Options for a task")]
struct TaskOptions {
    target_price: i32,
    ignore_membership: Option<String>,
    target_name: Option<String>,
    use_regex: bool,
}
pub struct Context {
    pub queue: Queue,
}

pub struct Query {}

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    // WARN: This feels pretty messy and not performant, but hey
    async fn tasks(_context: &Context) -> FieldResult<Vec<Task>> {
        let db = get_db_manager();
        let rows = db.query("SELECT * FROM fang_tasks", &[]).await.unwrap();

        let mut tasks = Vec::new();
        for row in rows {
            let task = ScalpingTask::try_from(&row).unwrap();
            let accounts = fetch_kide_accounts(task.account_ids).await.unwrap();

            tasks.push(Task {
                event_id: task.event_id,
                accounts,
                sale_start: task.sale_start,
            });
        }

        Ok(tasks)
    }

    async fn task(_context: &Context, event_id: String) -> FieldResult<Option<Task>> {
        let db = get_db_manager();
        let row = db
            .query_opt(
                "SELECT * FROM fang_tasks WHERE metadata->>'eventId' = $1",
                &[&event_id],
            )
            .await
            .unwrap();

        // check if we didn't get any rows
        if row.is_none() {
            return Ok(None);
        }

        let task = ScalpingTask::try_from(&row.unwrap()).unwrap();
        let accounts = fetch_kide_accounts(task.account_ids).await.unwrap();

        Ok(Some(Task {
            event_id: task.event_id,
            accounts,
            sale_start: task.sale_start,
        }))
    }

    async fn kide_accounts(_context: &Context) -> FieldResult<Vec<KideAccount>> {
        let accounts = fetch_all_kide_accounts().await.unwrap();
        Ok(accounts)
    }

    async fn kide_account(_context: &Context, id: String) -> FieldResult<Option<KideAccount>> {
        // TODO: Maybe separate fetch_kide_accounts into a fetch_kide_account and fetch_kide_accounts
        let accounts = fetch_kide_accounts(vec![id]).await.unwrap();
        Ok(accounts.into_iter().next())
    }
}

pub type Schema =
    juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
