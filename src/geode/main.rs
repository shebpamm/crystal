
use crystal::{Event, TicketSpecification};

struct TicketReservation {
    account: String,
    specification: TicketSpecification,
    event: Event,
}

fn main() {
    println!("Hello, world!");

    let test_event = Event {
        uid: "test".to_string(),
        name: "Test Event".to_string(),
        sales_start: chrono::Utc::now(),
    };
}
