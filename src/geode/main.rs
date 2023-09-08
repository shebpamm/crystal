use crystal::strategy::All;
use crystal::sale::Sale;

// struct TicketReservation {
//     account: String,
//     specification: TicketSpecification,
//     event: Event,
// }

fn main() {
    env_logger::init();

    let test_product = Sale::from_uid("22b2e772-5889-4b18-bae9-24a3d05bfe3f".to_string());


    test_product.reserve_all(&All);
}
