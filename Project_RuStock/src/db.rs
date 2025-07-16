use rusqlite::{params, Connection, Result, OptionalExtension};
use crate::product::Product;
use crate::sale::{Sale, SaleItem};
use crate::purchase::Purchase;
use crate::auth::Manager;
use chrono::Utc;

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
                id TEXT PRIMARY KEY,
                product_id TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                purchase_price REAL NOT NULL,
                total_cost REAL NOT NULL,
                purchase_date INTEGER NOT NULL,
                FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS managers (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                full_name TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 1
            )",
            [],
        )?;

        // Create default admin manager if no managers exist
        self.create_default_admin_if_needed()?;

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
            let mut stmt = tx.prepare("SELECT quantity FROM products WHERE id = ?")?;
            let current_quantity: i32 = stmt.query_row([&item.product_id], |row| row.get(0))?;

            if current_quantity < item.quantity {
                return Err(rusqlite::Error::InvalidParameterCount(0, 1));
            }

            tx.execute(
                "UPDATE products SET quantity = quantity - ? WHERE id = ?",
                params![item.quantity, item.product_id],
            )?;

            tx.execute(
                "INSERT INTO sales (product_id, quantity, total_price, sale_date) 
                 VALUES (?, ?, ?, datetime('now'))",
                params![
                    item.product_id,
                    item.quantity,
                    item.total_price,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn record_purchase(&mut self, purchase: &Purchase) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT INTO purchases (id, product_id, quantity, purchase_price, total_cost, purchase_date) 
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                purchase.id,
                purchase.product_id,
                purchase.quantity,
                purchase.purchase_price,
                purchase.total_cost,
                purchase.purchase_date,
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    #[allow(dead_code)]
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

    pub fn get_all_sales(&self) -> Result<Vec<(String, i32, f64, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT s.product_id, s.quantity, s.total_price, s.sale_date, p.name 
             FROM sales s
             JOIN products p ON s.product_id = p.id
             ORDER BY s.sale_date DESC"
        )?;

        let sales = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(4)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        sales.collect()
    }

    pub fn get_all_purchases(&self) -> Result<Vec<Purchase>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, product_id, quantity, purchase_price, total_cost, purchase_date 
             FROM purchases
             ORDER BY purchase_date DESC"
        )?;

        let purchases = stmt.query_map([], |row| {
            Ok(Purchase {
                id: row.get(0)?,
                product_id: row.get(1)?,
                quantity: row.get(2)?,
                purchase_price: row.get(3)?,
                total_cost: row.get(4)?,
                purchase_date: row.get(5)?,
            })
        })?;

        purchases.collect()
    }

    // Manager management functions
    fn create_default_admin_if_needed(&self) -> Result<()> {
        let count: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM managers",
            [],
            |row| row.get(0)
        )?;

        if count == 0 {
            let admin = Manager::new(
                "admin".to_string(),
                "admin123".to_string(),
                "System Administrator".to_string()
            );
            self.add_manager(&admin)?;
        }

        Ok(())
    }

    pub fn add_manager(&self, manager: &Manager) -> Result<()> {
        self.conn.execute(
            "INSERT INTO managers (id, username, password, full_name, created_at, is_active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                manager.id,
                manager.username,
                manager.password,
                manager.full_name,
                manager.created_at,
                manager.is_active as i32
            ],
        )?;
        Ok(())
    }

    pub fn get_manager_by_username(&self, username: &str) -> Result<Option<Manager>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password, full_name, created_at, is_active 
             FROM managers WHERE username = ?1"
        )?;
        
        let manager = stmt.query_row([username], |row| {
            Ok(Manager {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                full_name: row.get(3)?,
                created_at: row.get(4)?,
                is_active: row.get::<_, i32>(5)? == 1,
            })
        }).optional()?;

        Ok(manager)
    }

    pub fn get_all_managers(&self) -> Result<Vec<Manager>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password, full_name, created_at, is_active 
             FROM managers ORDER BY created_at DESC"
        )?;

        let managers = stmt.query_map([], |row| {
            Ok(Manager {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                full_name: row.get(3)?,
                created_at: row.get(4)?,
                is_active: row.get::<_, i32>(5)? == 1,
            })
        })?;

        managers.collect()
    }

    pub fn update_manager_status(&self, manager_id: &str, is_active: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE managers SET is_active = ?1 WHERE id = ?2",
            params![is_active as i32, manager_id],
        )?;
        Ok(())
    }

    pub fn authenticate_manager(&self, username: &str, password: &str) -> Result<Option<Manager>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password, full_name, created_at, is_active 
             FROM managers WHERE username = ?1 AND password = ?2 AND is_active = 1"
        )?;
        
        let manager = stmt.query_row([username, password], |row| {
            Ok(Manager {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                full_name: row.get(3)?,
                created_at: row.get(4)?,
                is_active: row.get::<_, i32>(5)? == 1,
            })
        }).optional()?;

        Ok(manager)
    }
} 