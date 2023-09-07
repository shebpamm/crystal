use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use reqwest::blocking::Get;

const KIDE_API_BASE_URL: &str = "https://api.kide.app/api/products/";

pub struct TicketSpecification {}

#[derive(Serialize, Deserialize)]
pub struct Event {
    uid: String,
    name: String,
    sales_start: DateTime<Utc>,
}

impl Event {
    pub fn from_uid(uid: String) -> Self {
        let url = format!("{}{}", KIDE_API_BASE_URL, uid);
        let response = reqwest::blocking::get(&url).unwrap();
        let event: Event = response.json().unwrap();
        event
    }
}
