use crate::api::Variant;
use crate::task::TaskOptions;
use std::cmp;
use std::cmp::Ordering;
use sublime_fuzzy::best_match;

const NEGATIVE_WORDS: [&str; 3] = [
    "allergia",
    "handicap",
    "inva",
];

const POSITIVE_WORDS: [&str; 4] = [
    "4 hengen",
    "Promenade",
    "A-hytti",
    "helga",
];

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
        let cap = cmp::min(
            variant.product_variant_maximum_item_quantity_per_user,
            variant.product_variant_maximum_reservable_quantity,
        );

        cmp::min(variant.availability, cap)
    }
}

#[derive(Debug, Clone)]
pub struct TicketPriorityStrategy {
    pub name_weight: i32,
    pub price_weight: i32,
    pub options: TaskOptions,
}

impl TicketPriorityStrategy {
    pub fn new(options: TaskOptions) -> Self {
        Self {
            name_weight: 1,
            price_weight: 1000,
            options,
        }
    }

    pub fn choose(&self, variants: &Vec<Variant>) -> Option<Variant> {
        let mut variants = variants.clone();

        // Filter out variants that are sold out
        variants.retain(|variant| variant.availability > 0);

        if !self.options.ignore_membership {
            // Filter out variants that require membership
            variants.retain(|variant| !variant.is_product_variant_membership_required);
        }

        variants.sort_by(|a, b| self.compare_variants(a.clone(), b.clone()));

        variants.first().cloned()
    }

    pub fn compare_variants(&self, a: Variant, b: Variant) -> cmp::Ordering {
        // Calculate scores based on weights and criteria
        let a_score = self.calculate_score(&a);
        let b_score = self.calculate_score(&b);

        log::trace!(
            "Comparing {} (score: {}) to {} (score: {})",
            a.name,
            a_score,
            b.name,
            b_score
        );

        // Compare scores
        if a_score > b_score {
            Ordering::Less
        } else if a_score < b_score {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn calculate_score(&self, variant: &Variant) -> i32 {
        let name_score = self.calculate_name_score(&variant.name);
        let price_score = self.calculate_price_score(variant.price_per_item);

        name_score * self.name_weight + price_score * self.price_weight
    }

    // Helper function to calculate the name score (fuzzy string comparison)
    fn calculate_name_score(&self, name: &str) -> i32 {
        Self::score_word(name).try_into().unwrap_or(0)
    }

    fn score_word(word: &str) -> isize {
        let mut positive_score = 0;
        let mut negative_score = 0;

        for positive_word in POSITIVE_WORDS.iter() {
            let score = match best_match(positive_word, word) {
                Some(m) => m.score(),
                None => 0,
            };

            positive_score += score;
        }

        for negative_word in NEGATIVE_WORDS.iter() {
            let score = match best_match(negative_word, word) {
                Some(m) => m.score(),
                None => 0,
            };

            negative_score += score;
        }

        positive_score - negative_score * 10
    }

    // Helper function to calculate the price score (exact price match)
    fn calculate_price_score(&self, price: i64) -> i32 {
        match self.options.target_price {
            Some(target_price) => {
                let target_price = target_price * 100;
                log::trace!(
                    "Comparing price {} to target price {}",
                    price,
                    target_price as i64
                );
                if price == target_price as i64 {
                    100
                } else {
                    0
                }
            }
            None => 0,
        }
    }
}
