use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Product {
    #[allow(dead_code)]
    pub fn new(name: String, description: String, price: f64, quantity: i32) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            price,
            quantity,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: Option<String>, description: Option<String>, price: Option<f64>, quantity: Option<i32>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(description) = description {
            self.description = description;
        }
        if let Some(price) = price {
            self.price = price;
        }
        if let Some(quantity) = quantity {
            self.quantity = quantity;
        }
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Product name cannot be empty".to_string());
        }
        if self.price < 0.0 {
            return Err("Product price cannot be negative".to_string());
        }
        if self.quantity < 0 {
            return Err("Product quantity cannot be negative".to_string());
        }
        Ok(())
    }
} 