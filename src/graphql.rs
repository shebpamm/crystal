use chrono::{DateTime, Utc};
use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

use fang::asynk::async_queue::AsyncQueueable;
use fang::AsyncRunnable;
use fang::FangTaskState;

use crate::account::KideAccount;
use crate::db::get_db_manager;
use crate::queue::Queue;
use crate::request::Client;
use crate::task::ScalpingTask;

// ---- Context ----

pub struct Context {
    pub queue: RwLock<Queue>,
}

impl juniper::Context for Context {}

// ---- Enums ----

#[derive(Debug, GraphQLEnum)]
#[graphql(description = "The state of a task")]
enum TaskState {
    New,
    InProgress,
    Finished,
    Failed,
    Retried,
}

impl From<FangTaskState> for TaskState {
    fn from(state: FangTaskState) -> Self {
        match state {
            FangTaskState::New => TaskState::New,
            FangTaskState::InProgress => TaskState::InProgress,
            FangTaskState::Finished => TaskState::Finished,
            FangTaskState::Failed => TaskState::Failed,
            FangTaskState::Retried => TaskState::Retried,
        }
    }
}

// ---- Errors ----

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Kide account not found: {0}")]
    KideAccountNotFound(Uuid),
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    #[error("Database error: {0}")]
    DBError(#[from] crate::db::DBError),
}

// ---- Query types ----

// Auxillary struct, that mirrors the ScalpingTask struct but instead provides KideAccounts and not account_ids and enriches the struct with status
#[derive(GraphQLObject)]
#[graphql(description = "A task")]
struct Task {
    event_id: String,
    accounts: Vec<KideAccount>,
    sale_start: DateTime<Utc>,
    state: TaskState,
    // options: TaskOptions,
}

impl Task {
    pub async fn try_from_scalping_task(task: ScalpingTask) -> Result<Self, ApiError> {
        let db = get_db_manager();
        let row = db
            .query_one(
                "SELECT * FROM fang_tasks WHERE metadata->>'eventId' = $1",
                &[&task.event_id],
            )
            .await?;

        // I don't bother inlining this
        let state: FangTaskState = row.get("state");
        let state: TaskState = state.into();

        Ok(Self {
            event_id: task.event_id,
            accounts: KideAccount::from_uuids(task.account_ids).await?,
            sale_start: task.sale_start,
            state
        })
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "Options for a task")]
struct TaskOptions {
    target_price: i32,
    ignore_membership: Option<String>,
    target_name: Option<String>,
    use_regex: bool,
}

// ---- Query Root ----

pub struct Query {}

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    // WARN: This feels pretty messy and not performant, but hey
    // Could be optimized by fetching the accounts only once and assigning here, but db operations
    // are cheap I guess...
    async fn tasks(_context: &Context) -> FieldResult<Vec<Task>> {
        let db = get_db_manager();
        let rows = db.query("SELECT * FROM fang_tasks", &[]).await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = ScalpingTask::try_from(&row)?;

            // I don't bother inlining this
            let state: FangTaskState = row.get("state");
            let state: TaskState = state.into();

            let accounts = KideAccount::from_uuids(task.account_ids).await?;

            tasks.push(Task {
                event_id: task.event_id,
                accounts,
                sale_start: task.sale_start,
                state,
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
            .await?;

        // check if we didn't get any rows
        if row.is_none() {
            return Ok(None);
        }

        let row = row.unwrap();

        let task = ScalpingTask::try_from(&row)?;
        let accounts = KideAccount::from_uuids(task.account_ids).await?;


        // I don't bother inlining this
        let state: FangTaskState = row.get("state");
        let state: TaskState = state.into();

        Ok(Some(Task {
            event_id: task.event_id,
            accounts,
            sale_start: task.sale_start,
            state,
        }))
    }

    async fn kide_accounts(_context: &Context) -> FieldResult<Vec<KideAccount>> {
        let accounts = KideAccount::all().await.unwrap();
        Ok(accounts)
    }

    async fn kide_account(_context: &Context, uuid: Uuid) -> FieldResult<Option<KideAccount>> {
        // TODO: Maybe separate KideAccount::from_uuid into a fetch_kide_account and fetch_kide_accounts
        let accounts = KideAccount::from_uuids(vec![uuid]).await.unwrap();
        Ok(accounts.into_iter().next())
    }
}

// ---- Mutation Inputs ----

#[derive(GraphQLInputObject)]
struct AddKideAccountInput {
    name: String,
    token: String,
}

#[derive(GraphQLInputObject)]
struct UpdateKideAccountInput {
    id: Uuid,
    name: Option<String>,
    token: Option<String>,
}

#[derive(GraphQLInputObject)]
struct DeleteKideAccountInput {
    id: Uuid,
}

#[derive(GraphQLInputObject)]
struct AddTaskInput {
    event_id: String,
    accounts: Vec<Uuid>,
    // options: TaskOptionsInput,
}

#[derive(GraphQLInputObject)]
struct UpdateTaskInput {
    event_id: String,
    accounts: Option<Vec<Uuid>>,
    // options: Option<TaskOptionsInput>,
}

// Used for both AddTaskInput and UpdateTaskInput
#[derive(GraphQLInputObject)]
struct TaskOptionsInput {
    target_price: i32,
    ignore_membership: Option<String>,
    target_name: Option<String>,
    use_regex: bool,
}

#[derive(GraphQLInputObject)]
struct DeleteTaskInput {
    event_id: String,
}

// ---- Mutation Root ----

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    async fn add_kide_account(
        _context: &Context,
        input: AddKideAccountInput,
    ) -> FieldResult<KideAccount> {
        // Implement the logic to add a new KideAccount using the input object
        Ok(KideAccount::create(input.name, input.token).await?)
    }

    async fn update_kide_account(
        _context: &Context,
        input: UpdateKideAccountInput,
    ) -> FieldResult<Option<KideAccount>> {
        let mut account = KideAccount::from_uuid(input.id)
            .await?
            .ok_or_else(|| ApiError::KideAccountNotFound(input.id))?;

        input.name.map(|name| account.name = name);
        input.token.map(|token| account.token = token);

        account.save().await?;

        Ok(Some(account))
    }

    async fn delete_kide_account(
        _context: &Context,
        input: DeleteKideAccountInput,
    ) -> FieldResult<Uuid> {
        KideAccount::delete(input.id).await?;
        Ok(input.id)
    }

    async fn add_task(context: &Context, input: AddTaskInput) -> FieldResult<Task> {
        // Fetch event details
        //
        // NOTE: Could be better to have a permanent client in the context, but load will be so
        // low that it doesn't matter
        let client = Client::new();
        let sale_client = client.product(input.event_id.clone()).await.unwrap();

        let task = ScalpingTask::new(
            input.event_id,
            input.accounts,
            sale_client.sale.product.date_sales_from,
        );

        // Lock the queue for writing
        let mut queue = context.queue.write().await;

        // Queue new task for workers
        queue
            .schedule_task(&task as &dyn AsyncRunnable)
            .await
            .unwrap();

        Ok(Task::try_from_scalping_task(task).await?)
    }

    async fn update_task(_context: &Context, input: UpdateTaskInput) -> FieldResult<Option<Task>> {
        // TODO: This much logic shouldn't be here
        let db = get_db_manager();
        let row = db
            .query_opt(
                "SELECT * FROM fang_tasks WHERE metadata->>'eventId' = $1",
                &[&input.event_id],
            )
            .await?;

        // check if we didn't get any rows
        if row.is_none() {
            return Ok(None);
        }

        let row = row.unwrap();


        // I don't bother inlining this
        let state: FangTaskState = row.get("state");
        let state: TaskState = state.into();

        let mut task = ScalpingTask::try_from(&row)?;
        input.accounts.map(|accounts| task.account_ids = accounts);

        let metadata = serde_json::to_value(&task as &dyn AsyncRunnable)?;

        let _ = db
            .execute(
                "UPDATE fang_tasks SET metadata = $1 WHERE metadata->>'eventId' = $2",
                &[&metadata, &task.event_id],
            )
            .await;

        let accounts = KideAccount::from_uuids(task.account_ids).await?;

        Ok(Some(Task {
            event_id: task.event_id,
            accounts,
            sale_start: task.sale_start,
            state,
        }))
    }

    async fn delete_task(_context: &Context, input: DeleteTaskInput) -> FieldResult<String> {
        let db = get_db_manager();
        let _ = db
            .execute(
                "DELETE FROM fang_tasks WHERE metadata->>'eventId' = $1",
                &[&input.event_id],
            )
            .await?;

        Ok(input.event_id)
    }
}

// ---- Schema ----

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
