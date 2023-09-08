use crystal::strategy::All;
use crystal::request::Client;

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = Client::new();

    let product = client.product("22b2e772-5889-4b18-bae9-24a3d05bfe3f".to_string())
        .await
        .unwrap();

    product.reserve_all(&All).await;
}
