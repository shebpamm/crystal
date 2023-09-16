use crystal::db::initialize_db_manager;
use crystal::graphql::{Context, Query, Schema};
use crystal::queue::connect_to_queue;
use dotenvy::dotenv;
use juniper::{EmptyMutation, EmptySubscription, Variables};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set");

    log::info!("Starting...");

    // Initialize DB Pool for crystal operations
    initialize_db_manager(database_url.clone()).await;

    let queue = connect_to_queue(database_url).await;

    let ctx = Context { queue };
    let schema = Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new());

    let (res, errors) = juniper::execute(
        "query { 
            kideAccount(uuid: \"c749d6d4-3ede-44b1-b4e6-20b1f52b6a2c\") { 
                uuid 
                name 
                token 
            } 
        }",
        None,
        &schema,
        &Variables::new(),
        &ctx,
    )
    .await
    .unwrap();

    println!("res: {:#?}", res);
    println!("errors: {:#?}", errors);
}
