use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: String,
    pub name: String,
    // pub description: String,
    pub media_filename: String,
    pub favorited_times: i64,
    pub product_type: i64,
    pub city: String,
    pub country: String,
    pub date_actual_from: DateTime<Utc>,
    pub date_actual_until: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub postal_code: String,
    pub street_address: String,
    pub place: String,
    pub company_id: String,
    pub date_publish_from: DateTime<Utc>,
    pub date_publish_until: DateTime<Utc>,
    pub date_sales_from: DateTime<Utc>,
    pub date_sales_until: DateTime<Utc>,
    pub is_favorited: bool,
    pub availability: i64,
    #[serde(default)]
    pub max_price: Price,
    #[serde(default)]
    pub min_price: Price,
    pub has_free_inventory_items: bool,
    pub has_inventory_items: bool,
    pub is_long: bool,
    pub is_actual: bool,
    pub sales_started: bool,
    pub sales_ended: bool,
    pub sales_ongoing: bool,
    pub sales_paused: bool,
    pub time: i64,
    pub time_until_sales_start: i64,
    #[serde(default)]
    pub max_total_reservations_per_checkout: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub eur: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub inventory_id: String,
    pub currency_code: String,
    pub price_per_item: i64,
    pub vat: i64,
    pub availability: i64,
    pub is_product_variant_haka_authentication_required: bool,
    pub is_product_variant_transferable: bool,
    pub product_variant_maximum_item_quantity_per_user: i64,
    pub product_variant_maximum_reservable_quantity: i64,
    pub product_variant_minimum_reservable_quantity: i64,
    pub access_control_memberships: Option<Vec<AccessControlMembership>>,
    pub product_id: String,
    pub product_type: i64,
    pub date_sales_from: DateTime<Utc>,
    pub is_product_variant_membership_required: bool,
    pub is_product_variant_student_card_required: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessControlMembership {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub media_filename: String,
    pub form: String,
    pub is_disabled: bool,
    pub is_initially_disabled: bool,
    pub can_be_activated_externally: bool,
    pub granted_by: Vec<GrantedBy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantedBy {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub media_filename: Option<String>,
    pub inventory_id: String,
    pub currency_code: String,
    pub price_per_item: i64,
    pub vat: i64,
    pub notes_instructions: String,
    pub availability: i64,
    pub is_product_variant_haka_authentication_required: bool,
    pub is_product_variant_transferable: bool,
    pub product_variant_maximum_item_quantity_per_user: i64,
    pub product_variant_maximum_reservable_quantity: i64,
    pub product_variant_minimum_reservable_quantity: i64,
    pub contents_memberships: Vec<ContentsMembership>,
    pub product_id: String,
    pub product_type: i64,
    pub is_product_variant_membership_required: bool,
    pub is_product_variant_student_card_required: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentsMembership {
    pub id: String,
    pub name: String,
    pub form: String,
    pub can_be_activated_externally: bool,
    pub has_terms_of_use: bool,
    pub has_privacy_policy: bool,
    pub is_disabled: bool,
    pub is_initially_disabled: bool,
    pub binding_valid_from: String,
    pub binding_valid_until: String,
    pub validity_period_in_days: Value,
    pub membership_provides_physical_card: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub name: String,
    pub ordering_number: i64,
}

