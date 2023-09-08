use crate::sale::Sale;
use std::env;
use serde::{Deserialize, Serialize};

const KIDE_API_BASE_URL: &str = "https://api.kide.app/api/";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponse {
    pub model: Sale,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchReservation {
    pub to_create: Vec<VariantReservation>,
    pub to_cancel: Vec<VariantReservation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantReservation {
    pub inventory_id: String,
    pub quantity: i64,
}

impl BatchReservation {
    pub fn create(variant: &VariantReservation) -> Self {
        BatchReservation {
            to_create: vec![variant.clone()],
            to_cancel: vec![],
        }
    }

    pub fn cancel(variant: &VariantReservation) -> Self {
        BatchReservation {
            to_create: vec![],
            to_cancel: vec![variant.clone()],
        }
    }
}

fn token() -> String {
    match env::var("KIDE_API_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("KIDE_API_TOKEN not set"),
    } 
}

pub fn products(uid: String) -> Sale {
    let url = format!("{}products/{}", KIDE_API_BASE_URL, uid);
    let response = reqwest::blocking::get(&url).unwrap();
    let response_document: ProductResponse = response.json().unwrap();

    return response_document.model;
}

pub fn reserve(reservation: &BatchReservation) {
    log::debug!("Reserving reservation: {:?}", reservation);

    let url = format!("{}reservations", KIDE_API_BASE_URL);

    let client = reqwest::blocking::Client::new();
    let response = client.post(&url)
        .header("Authorization", format!("Bearer {}", token()))
        .json(reservation)
        .send()
        .unwrap();

    log::debug!("Response: {:#?}", response);
}
