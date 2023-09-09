use crystal::strategy::{Single,All,Count};
use crystal::request::Client;
use crystal::sale::SaleClient;
use tokio::task::JoinSet;

async fn test_get(client: Client) {
        let p = client.product("22b2e772-5889-4b18-bae9-24a3d05bfe3f".to_string()).await.unwrap();
        println!("{}", p.sale.product.name);
}

async fn test_reserve(sc: SaleClient, count: i64) {
    sc.reserve_all(&Count { count }).await;
    println!("Reserved {}", count);
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = Client::new();
    let mut set = JoinSet::new();


    let product = client.product("22b2e772-5889-4b18-bae9-24a3d05bfe3f".to_string())
        .await
        .unwrap();

    for i in 1..20 {
        set.spawn(test_reserve(product.clone(), i));
    }

    while let Some(_res) = set.join_next().await {

    }
}
