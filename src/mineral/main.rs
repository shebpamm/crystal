use crystal::db::initialize_db_manager;
use crystal::graphql::{Context, Query, Mutation, Schema};
use crystal::queue::connect_to_queue;
use dotenvy::dotenv;
use juniper::EmptySubscription;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware,
    route,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, HttpRequest
};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

#[route("/graphiql", method = "GET")]
async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

#[route("/playground", method = "GET")]
async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Arc<Schema>>,
    ctx: web::Data<Arc<Context>>,
    ) -> Result<HttpResponse, Error> {
        graphql_handler(&schema, &ctx, req, payload).await
}

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("PROD_DATABASE_URL").expect("PROD_DATABASE_URL must be set");

    log::info!("Starting...");

    // Initialize DB Pool for crystal operations
    initialize_db_manager(database_url.clone()).await;

    let queue = RwLock::new(connect_to_queue(database_url).await);
    let ctx = Arc::new(Context { queue });

    let schema = Arc::new(Schema::new(
        Query {},
        Mutation {},
        EmptySubscription::new(),
    ));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(ctx.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(graphql_route)
            .service(graphiql_route)
            .service(playground_route)
    });

    server.bind("127.0.0.1:8080").unwrap().run().await.unwrap();
}
