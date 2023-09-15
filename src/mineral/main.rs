use crystal::schema::{Context, DatabasePool, Mutation, Query, Schema};
use juniper::{EmptySubscription, Variables};

fn main() {
    let pool = DatabasePool {};
    let ctx = Context { pool };
    let schema = Schema::new(Query, Mutation {}, EmptySubscription::new());

    let (res, errors) = juniper::execute_sync(
        "query { task(id : \"123\") { eventId } }",
        None,
        &schema,
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    println!("res: {:#?}", res);
    println!("errors: {:#?}", errors);
}
