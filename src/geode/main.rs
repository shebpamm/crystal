use crystal::strategy::All;
use crystal::request::Client;

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = Client::new();

    let product = client.product("94199b1a-80b1-4e24-975f-ae8c6759aecd".to_string())
        .await
        .unwrap();

    product.reserve_all(&All).await;
}
