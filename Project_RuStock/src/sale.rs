use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaleItem {
    pub product_id: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sale {
    pub id: String,
    pub items: Vec<SaleItem>,
    pub total_amount: f64,
    pub total_profit: f64,
    pub timestamp: i64,
}

impl Sale {
    pub fn new(items: Vec<SaleItem>) -> Self {
        let total_amount = items.iter().map(|item| item.total_price).sum();
        Self {
            id: Uuid::new_v4().to_string(),
            items,
            total_amount,
            total_profit: 0.0,
            timestamp: Utc::now().timestamp(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.items.is_empty() {
            return Err("Sale must have at least one item".to_string());
        }
        for item in &self.items {
            if item.quantity <= 0 {
                return Err("Item quantity must be positive".to_string());
            }
            if item.unit_price <= 0.0 {
                return Err("Item unit price must be positive".to_string());
            }
            if (item.unit_price * item.quantity as f64 - item.total_price).abs() > 0.01 {
                return Err("Item total price calculation mismatch".to_string());
            }
        }
        Ok(())
    }
} 