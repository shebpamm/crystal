use crate::api::Variant;
use std::cmp;

pub trait Quantity {
    fn quantity(&self, variant: &Variant) -> i64;
}

pub struct Single;

impl Quantity for Single {
    fn quantity(&self, _variant: &Variant) -> i64 {
        1
    }
}

pub struct Count {
    pub count: i64,
}

impl Quantity for Count {
    fn quantity(&self, _variant: &Variant) -> i64 {
        self.count
    }
}

pub struct All;

impl Quantity for All {
    fn quantity(&self, variant: &Variant) -> i64 {
        let cap = cmp::min(variant.product_variant_maximum_item_quantity_per_user, variant.product_variant_maximum_reservable_quantity);

        cmp::min(variant.availability, cap)
    }
}
