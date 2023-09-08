use crystal::request::Client;
use tokio::task::JoinSet;

async fn get(client: Client) {
        let p = client.product("22b2e772-5889-4b18-bae9-24a3d05bfe3f".to_string()).await.unwrap();
        println!("{}", p.sale.product.name);
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut set = JoinSet::new();

    for _ in 1..100 {
        set.spawn(get(client.clone()));
    }

    while let Some(_res) = set.join_next().await {

    }
}
