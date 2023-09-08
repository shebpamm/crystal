use crate::api::{Category, Company, Product, Variant};
use crate::request::{BatchReservation, VariantReservation};
use crate::strategy::Quantity;
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

impl Sale {
    pub fn from_uid(uid: String) -> Self {
        crate::request::products(uid)
    }

    pub fn reserve(&self, variant: &Variant, strategy: &impl Quantity) {
        let variant_reservation = variant.to_reservation(strategy);

        let batch = BatchReservation::create(&variant_reservation);

        crate::request::reserve(&batch)
    }

    pub fn reserve_all(&self, strategy: &impl Quantity) {
        let reservations = self
            .variants
            .iter()
            .map(|variant| variant.to_reservation(strategy))
            .collect();

        let batch = BatchReservation {
            to_create: reservations,
            to_cancel: vec![],
        };

        crate::request::reserve(&batch)
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
