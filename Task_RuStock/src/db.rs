use rusqlite::{Connection, Result, params, OptionalExtension};
use crate::product::Product;
use crate::sale::{Sale, SaleItem};
use chrono::Utc;

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

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sales (
                id TEXT PRIMARY KEY,
                total_amount REAL NOT NULL,
                total_profit REAL NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sale_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sale_id TEXT NOT NULL,
                product_id TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                unit_price REAL NOT NULL,
                total_price REAL NOT NULL,
                FOREIGN KEY (sale_id) REFERENCES sales(id),
                FOREIGN KEY (product_id) REFERENCES products(id)
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

    pub fn record_sale(&mut self, sale: &Sale) -> Result<()> {
        let tx = self.conn.transaction()?;

        // Update product quantity
        let rows_updated = tx.execute(
            "UPDATE products SET quantity = quantity - ? WHERE id = ? AND quantity >= ?",
            params![sale.quantity, sale.product_id, sale.quantity],
        )?;

        if rows_updated == 0 {
            return Err(rusqlite::Error::InvalidParameterCount(0, 1));
        }

        // Check if quantity is now 0 and delete the product if it is
        let rows_deleted = tx.execute(
            "DELETE FROM products WHERE id = ? AND quantity = 0",
            params![sale.product_id],
        )?;

        // Record the sale
        tx.execute(
            "INSERT INTO sales (product_id, quantity, total_price, sale_date) VALUES (?, ?, ?, datetime('now'))",
            params![
                sale.product_id,
                sale.quantity,
                sale.quantity as f64 * sale.price,
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn get_sale(&self, id: &str) -> Result<Option<Sale>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, total_amount, total_profit, timestamp FROM sales WHERE id = ?1"
        )?;

        let sale = stmt.query_row([id], |row| {
            let sale_id: String = row.get(0)?;
            
            let mut stmt = self.conn.prepare(
                "SELECT product_id, quantity, unit_price, total_price 
                 FROM sale_items WHERE sale_id = ?1"
            )?;
            
            let items: Result<Vec<SaleItem>> = stmt.query_map([&sale_id], |row| {
                Ok(SaleItem {
                    product_id: row.get(0)?,
                    quantity: row.get(1)?,
                    unit_price: row.get(2)?,
                    total_price: row.get(3)?,
                })
            })?.collect();

            Ok(Sale {
                id: sale_id,
                items: items?,
                total_amount: row.get(1)?,
                total_profit: row.get(2)?,
                timestamp: row.get(3)?,
            })
        }).optional()?;

        Ok(sale)
    }

    pub fn get_all_sales(&self) -> Result<Vec<Sale>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, total_amount, total_profit, timestamp FROM sales ORDER BY timestamp DESC"
        )?;

        let sales_iter = stmt.query_map([], |row| {
            let sale_id: String = row.get(0)?;
            
            let mut stmt = self.conn.prepare(
                "SELECT product_id, quantity, unit_price, total_price 
                 FROM sale_items WHERE sale_id = ?1"
            )?;
            
            let items: Result<Vec<SaleItem>> = stmt.query_map([&sale_id], |row| {
                Ok(SaleItem {
                    product_id: row.get(0)?,
                    quantity: row.get(1)?,
                    unit_price: row.get(2)?,
                    total_price: row.get(3)?,
                })
            })?.collect();

            Ok(Sale {
                id: sale_id,
                items: items?,
                total_amount: row.get(1)?,
                total_profit: row.get(2)?,
                timestamp: row.get(3)?,
            })
        })?;

        let mut sales = Vec::new();
        for sale in sales_iter {
            sales.push(sale?);
        }
        Ok(sales)
    }
} 