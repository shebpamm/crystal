use crystal::request::Client;

fn main() {
    let client = Client::new();

    for _ in 1..100 {
        let p = client.product("22b2e772-5889-4b18-bae9-24a3d05bfe3f".to_string());
        println!("{}", p.sale.product.name);
    }
}
