use crystal::account::AccountIDList;
use crystal::db::initialize_db_manager;
use crystal::queue::connect_to_queue;
use crystal::request::Client;
use crystal::task::ScalpingTask;

use dotenvy::dotenv;
use fang::asynk::async_queue::AsyncQueueable;
use fang::AsyncRunnable;
use std::env;
use uuid::uuid;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Task {
        // Run locally and don't delegate to queue
        #[clap(short, long)]
        direct: bool,

        // Event URL
        url: String,
    },
    Account {
        // Nickname for account
        name: String,

        // JWT Token for account
        token: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let cli = Cli::parse();

    let database_url = env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set");

    log::info!("Starting...");

    // Initialize DB Pool for crystal operations
    initialize_db_manager(database_url.clone()).await;

    match cli.command {
        Commands::Task { url, direct } => {
            let event_id = url.split("/").last().unwrap();
            let account_uuids = vec![
                uuid!("58ce05ca-5c43-44d5-a5a7-a4b5a727b6ad"),
                uuid!("c749d6d4-3ede-44b1-b4e6-20b1f52b6a2c"),
            ];

            // Run locally?
            if direct {
                run_task(event_id.to_string(), account_uuids).await;
            } else {
                add_task(event_id.to_string(), account_uuids, database_url).await;
            }
        }
        Commands::Account { name, token } => {
            crystal::account::KideAccount::create(name, token)
                .await
                .unwrap();
        }
    }
}

async fn run_task(event_id: String, account_ids: AccountIDList) {
    crystal::scalp::scalp(event_id.to_string(), account_ids, Default::default())
        .await
        .unwrap();
}

async fn add_task(event_id: String, account_ids: AccountIDList, database_url: String) {
    // Connect & create pool to task queue
    let mut queue = connect_to_queue(database_url).await;
    log::info!("Queue connected...");

    // Fetch event details
    let client = Client::new();
    let sale_client = client.product(event_id.to_string()).await.unwrap();

    // Queue new task for workers
    let task = ScalpingTask::new(
        event_id.to_string(),
        account_ids,
        sale_client.sale.product.date_sales_from,
        Default::default(),
    );

    queue
        .schedule_task(&task as &dyn AsyncRunnable)
        .await
        .unwrap();
}
