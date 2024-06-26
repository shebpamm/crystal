use crate::api::{Category, Company, Product, Variant};
use crate::request::{BatchReservation, Client, VariantReservation};
use crate::strategy::{Quantity, TicketPriorityStrategy};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sale {
    pub company: Company,
    pub product: Product,
    pub variants: Vec<Variant>,
    pub categories: Vec<Category>,
    pub is_haka_required: bool,
}

#[derive(Default, Debug, Clone)]
pub struct SaleClient {
    pub sale: Sale,
    pub client: Client,
}

impl SaleClient {
    pub async fn reserve_fuzzy(&self, token: String, strategy: &impl Quantity, priority_strategy: &TicketPriorityStrategy) {
        // This could be performed in scalp.rs a single time, let's do that if performance suffers

        let variant = priority_strategy.choose(&self.sale.variants);

        if let Some(variant) = variant {
            self.reserve(&variant, token, strategy).await;
        } else {
            println!("No variants to reserve");
        }
    }

    pub async fn reserve(&self, variant: &Variant, token: String, strategy: &impl Quantity) {
        let variant_reservation = variant.to_reservation(strategy);

        let batch = BatchReservation::create(&variant_reservation);

        match self.client.reserve(&batch, token).await {
            Ok(_) => println!("Reserved variant {}", variant.inventory_id),
            Err(e) => println!("Error: {}", e), // For now we don't care about errors, in future
                                                // we'd have retry logic
        }
    }

    pub async fn reserve_all(&self, token: String, strategy: &impl Quantity) {
        let mut total_quantity = 0;
        let reservations: Vec<VariantReservation> = self
            .sale
            .variants
            .iter()
            .filter_map(|variant| {
                if variant.availability > 0 {
                    let reservation = variant.to_reservation(strategy);
                    total_quantity += reservation.quantity;

                    Some(reservation)
                } else {
                    None
                }
            })
            .collect();

        if self.sale.product.max_total_reservations_per_checkout > 0
            && total_quantity > self.sale.product.max_total_reservations_per_checkout
        {
            println!(
                "Total quantity {} exceeds max total reservations per checkout {}",
                total_quantity, self.sale.product.max_total_reservations_per_checkout
            );
        }

        if reservations.is_empty() {
            println!("No variants to reserve");
            return;
        }

        let batch = BatchReservation {
            to_create: reservations,
            to_cancel: vec![],
        };

        match self.client.reserve(&batch, token).await {
            Ok(_) => log::debug!("Reserved all variants"),
            Err(e) => println!("Error: {}", e),
        }
    }
}

impl Variant {
    pub fn to_reservation(&self, strategy: &impl Quantity) -> VariantReservation {
        VariantReservation {
            inventory_id: self.inventory_id.clone(),
            quantity: strategy.quantity(self),
        }
    }
}
