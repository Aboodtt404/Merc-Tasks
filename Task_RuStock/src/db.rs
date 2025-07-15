use rusqlite::{Connection, Result, params, OptionalExtension};
use crate::product::Product;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("rustock.db")?;
        let db = Database { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                price REAL NOT NULL,
                quantity INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_product(&self, product: &Product) -> Result<()> {
        self.conn.execute(
            "INSERT INTO products (id, name, description, price, quantity, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                product.id,
                product.name,
                product.description,
                product.price,
                product.quantity,
                product.created_at,
                product.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn update_product(&self, product: &Product) -> Result<()> {
        self.conn.execute(
            "UPDATE products 
             SET name = ?1, description = ?2, price = ?3, quantity = ?4, updated_at = ?5
             WHERE id = ?6",
            params![
                product.name,
                product.description,
                product.price,
                product.quantity,
                product.updated_at,
                product.id
            ],
        )?;
        Ok(())
    }

    pub fn delete_product(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM products WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_product(&self, id: &str) -> Result<Option<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, price, quantity, created_at, updated_at 
             FROM products WHERE id = ?1"
        )?;
        
        let product = stmt.query_row([id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                price: row.get(3)?,
                quantity: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        }).optional()?;

        Ok(product)
    }

    pub fn get_all_products(&self) -> Result<Vec<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, price, quantity, created_at, updated_at 
             FROM products"
        )?;
        
        let products = stmt.query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                price: row.get(3)?,
                quantity: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut result = Vec::new();
        for product in products {
            result.push(product?);
        }
        Ok(result)
    }
} 