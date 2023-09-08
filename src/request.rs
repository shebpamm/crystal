use crate::sale::{Sale,SaleClient};
use serde::{Deserialize, Serialize};
use std::env;

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

#[derive(Default, Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    token: String,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
            token: Self::token(),
        }
    }

    fn token() -> String {
        match env::var("KIDE_API_TOKEN") {
            Ok(token) => token,
            Err(_) => panic!("KIDE_API_TOKEN not set"),
        }
    }

    pub async fn product(&self, uid: String) -> Result<SaleClient, reqwest::Error> {
        let url = format!("{}products/{}", KIDE_API_BASE_URL, uid);
        let response = self.client
            .get(&url)
            .send()
            .await?;
        let response_document: ProductResponse = response.json().await?;

        return Ok(SaleClient {
            sale: response_document.model,
            client: self.clone(),
        });
    }

    pub async fn reserve(&self, reservation: &BatchReservation) -> Result<(), reqwest::Error> {
        log::debug!("Reserving reservation: {:?}", reservation);

        let url = format!("{}reservations", KIDE_API_BASE_URL);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(reservation)
            .send()
            .await?;

        log::debug!("Response: {:#?}", response);

        Ok(())
    }
}
