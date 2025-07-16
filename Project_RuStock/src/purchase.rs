use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Purchase {
    pub id: String,
    pub product_id: String,
    pub quantity: i32,
    pub purchase_price: f64,
    pub total_cost: f64,
    pub purchase_date: i64,
}

impl Purchase {
    pub fn new(product_id: String, quantity: i32, purchase_price: f64) -> Self {
        let total_cost = purchase_price * quantity as f64;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            product_id,
            quantity,
            purchase_price,
            total_cost,
            purchase_date: chrono::Utc::now().timestamp(),
        }
    }
} 