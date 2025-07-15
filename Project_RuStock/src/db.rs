use rusqlite::{params, Connection, Result, OptionalExtension};
use crate::product::Product;
use crate::sale::{Sale, SaleItem};
use chrono::Utc;

pub struct Purchase {
    pub product_id: String,
    pub quantity: i32,
    pub purchase_price: f64,
    pub total_cost: f64,
    pub purchase_date: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Database> {
        let conn = Connection::open("rustock.db")?;
        let db = Database { conn };
        db.init_db()?;
        Ok(db)
    }

    fn init_db(&self) -> Result<()> {
        self.conn.execute("PRAGMA foreign_keys = ON", [])?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                price REAL NOT NULL,
                quantity INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sales (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                product_id TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                total_price REAL NOT NULL,
                sale_date TEXT NOT NULL,
                FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS purchases (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                product_id TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                purchase_price REAL NOT NULL,
                total_cost REAL NOT NULL,
                purchase_date TEXT NOT NULL,
                FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE CASCADE
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

    pub fn delete_product(&mut self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM products WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_product(&self, id: &str) -> Result<Option<Product>> {
        let mut stmt = self.conn.prepare("SELECT * FROM products WHERE id = ?")?;
        let product = stmt.query_row([id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                price: row.get(3)?,
                quantity: row.get(4)?,
                created_at: Utc::now().timestamp(),
                updated_at: Utc::now().timestamp(),
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

    pub fn get_products(&self) -> Result<Vec<Product>> {
        let mut stmt = self.conn.prepare("SELECT * FROM products")?;
        let products = stmt.query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                price: row.get(3)?,
                quantity: row.get(4)?,
                created_at: Utc::now().timestamp(),
                updated_at: Utc::now().timestamp(),
            })
        })?;

        products.collect()
    }

    pub fn record_sale(&mut self, sale: &Sale) -> Result<()> {
        let tx = self.conn.transaction()?;

        for item in &sale.items {
            let rows_updated = tx.execute(
                "UPDATE products SET quantity = quantity - ? WHERE id = ? AND quantity >= ?",
                params![item.quantity, item.product_id, item.quantity],
            )?;

            if rows_updated == 0 {
                return Err(rusqlite::Error::InvalidParameterCount(0, 1));
            }

            tx.execute(
                "DELETE FROM products WHERE id = ? AND quantity = 0",
                params![item.product_id],
            )?;
        }

        tx.execute(
            "INSERT INTO sales (product_id, quantity, total_price, sale_date) VALUES (?, ?, ?, datetime('now'))",
            params![
                sale.items[0].product_id,
                sale.items[0].quantity,
                sale.total_amount,
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn record_purchase(&mut self, purchase: &Purchase) -> Result<()> {
        let tx = self.conn.transaction()?;

        let rows_updated = tx.execute(
            "UPDATE products SET quantity = quantity + ? WHERE id = ?",
            params![purchase.quantity, purchase.product_id],
        )?;

        if rows_updated == 0 {
            return Err(rusqlite::Error::InvalidParameterCount(0, 1));
        }

        tx.execute(
            "INSERT INTO purchases (product_id, quantity, purchase_price, total_cost, purchase_date) 
             VALUES (?, ?, ?, ?, datetime('now'))",
            params![
                purchase.product_id,
                purchase.quantity,
                purchase.purchase_price,
                purchase.total_cost,
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

    pub fn get_purchases(&self) -> Result<Vec<(Purchase, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT p.product_id, p.quantity, p.purchase_price, p.total_cost, p.purchase_date, pr.name 
             FROM purchases p 
             JOIN products pr ON p.product_id = pr.id 
             ORDER BY p.purchase_date DESC"
        )?;

        let purchases = stmt.query_map([], |row| {
            Ok((
                Purchase {
                    product_id: row.get(0)?,
                    quantity: row.get(1)?,
                    purchase_price: row.get(2)?,
                    total_cost: row.get(3)?,
                    purchase_date: row.get(4)?,
                },
                row.get(5)?,
            ))
        })?;

        purchases.collect()
    }
} 