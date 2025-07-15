mod product;
mod db;
mod sale;

use std::io::{self, Write};
use chrono::{DateTime, Utc};
use product::Product;
use sale::{Sale, SaleItem};
use db::Database;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn display_logo() {
    println!(r#"
__________       _________ __                 __    
\______   \__ __/   _____//  |_  ____   ____ |  | __
 |       _/  |  \_____  \\   __\/  _ \_/ ___\|  |/ /
 |    |   \  |  /        \|  | (  <_> )  \___|    < 
 |____|_  /____/_______  /|__|  \____/ \___  >__|_ \
        \/             \/                  \/     \/                                           
    RuSTOCK Inventory Management System
    "#);
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_product(db: &Database) {
    clear_screen();
    display_logo();
    println!("\nAdd New Product");
    println!("---------------");

    let name = prompt("Product Name: ");
    let description = prompt("Description: ");
    let price = prompt("Price: ").parse::<f64>().unwrap_or(-1.0);
    let quantity = prompt("Quantity: ").parse::<i32>().unwrap_or(-1);

    let product = Product::new(name, description, price, quantity);

    match product.validate() {
        Ok(()) => {
            match db.add_product(&product) {
                Ok(()) => println!("\nProduct added successfully!"),
                Err(e) => eprintln!("\nError adding product: {}", e),
            }
        }
        Err(e) => eprintln!("\nValidation error: {}", e),
    }

    println!("\nPress Enter to continue...");
    prompt("");
}

fn list_products(db: &Database) {
    clear_screen();
    display_logo();
    println!("\nProduct List");
    println!("------------");

    match db.get_all_products() {
        Ok(products) => {
            if products.is_empty() {
                println!("No products found.");
            } else {
                for product in products {
                    println!("\nID: {}", product.id);
                    println!("Name: {}", product.name);
                    println!("Description: {}", product.description);
                    println!("Price: ${:.2}", product.price);
                    println!("Quantity: {}", product.quantity);
                    println!("------------------------");
                }
            }
        }
        Err(e) => eprintln!("Error fetching products: {}", e),
    }

    println!("\nPress Enter to continue...");
    prompt("");
}

fn edit_product(db: &Database) {
    clear_screen();
    display_logo();
    println!("\nEdit Product");
    println!("-----------");

    let id = prompt("Enter Product ID: ");
    
    match db.get_product(&id) {
        Ok(Some(mut product)) => {
            println!("\nCurrent Product Details:");
            println!("Name: {}", product.name);
            println!("Description: {}", product.description);
            println!("Price: ${:.2}", product.price);
            println!("Quantity: {}", product.quantity);
            println!("\nEnter new details (press Enter to keep current value):");

            let name = prompt("New Name: ");
            let description = prompt("New Description: ");
            let price_str = prompt("New Price: ");
            let quantity_str = prompt("New Quantity: ");

            let name = if name.is_empty() { None } else { Some(name) };
            let description = if description.is_empty() { None } else { Some(description) };
            let price = price_str.parse::<f64>().ok();
            let quantity = quantity_str.parse::<i32>().ok();

            product.update(name, description, price, quantity);

            match product.validate() {
                Ok(()) => {
                    match db.update_product(&product) {
                        Ok(()) => println!("\nProduct updated successfully!"),
                        Err(e) => eprintln!("\nError updating product: {}", e),
                    }
                }
                Err(e) => eprintln!("\nValidation error: {}", e),
            }
        }
        Ok(None) => println!("\nProduct not found."),
        Err(e) => eprintln!("\nError fetching product: {}", e),
    }

    println!("\nPress Enter to continue...");
    prompt("");
}

fn delete_product(db: &Database) {
    clear_screen();
    display_logo();
    println!("\nDelete Product");
    println!("-------------");

    let id = prompt("Enter Product ID: ");
    
    match db.get_product(&id) {
        Ok(Some(product)) => {
            println!("\nProduct Details:");
            println!("Name: {}", product.name);
            println!("Description: {}", product.description);
            println!("Price: ${:.2}", product.price);
            println!("Quantity: {}", product.quantity);

            let confirm = prompt("\nAre you sure you want to delete this product? (y/N): ");
            if confirm.to_lowercase() == "y" {
                match db.delete_product(&id) {
                    Ok(()) => println!("\nProduct deleted successfully!"),
                    Err(e) => eprintln!("\nError deleting product: {}", e),
                }
            } else {
                println!("\nDeletion cancelled.");
            }
        }
        Ok(None) => println!("\nProduct not found."),
        Err(e) => eprintln!("\nError fetching product: {}", e),
    }

    println!("\nPress Enter to continue...");
    prompt("");
}

fn record_sale(db: &mut Database) {
    clear_screen();
    display_logo();
    println!("\nRecord Sale");
    println!("-----------");

    let mut sale_items = Vec::new();
    loop {
        match db.get_all_products() {
            Ok(products) => {
                if products.is_empty() {
                    println!("No products available.");
                    return;
                }
                println!("\nAvailable Products:");
                println!("------------------");
                for product in &products {
                    println!("ID: {}", product.id);
                    println!("Name: {}", product.name);
                    println!("Price: ${:.2}", product.price);
                    println!("Available Quantity: {}", product.quantity);
                    println!("------------------");
                }

                let product_id = prompt("Enter Product ID (or press Enter to finish): ");
                if product_id.is_empty() {
                    break;
                }

                if let Ok(Some(product)) = db.get_product(&product_id) {
                    let quantity_str = prompt("Enter quantity: ");
                    if let Ok(quantity) = quantity_str.parse::<i32>() {
                        if quantity <= 0 {
                            println!("Quantity must be positive.");
                            continue;
                        }
                        if quantity > product.quantity {
                            println!("Insufficient stock. Available: {}", product.quantity);
                            continue;
                        }

                        let unit_price = product.price;
                        let total_price = unit_price * quantity as f64;

                        sale_items.push(SaleItem {
                            product_id: product.id,
                            quantity,
                            unit_price,
                            total_price,
                        });

                        println!("Item added to sale.");
                    } else {
                        println!("Invalid quantity.");
                    }
                } else {
                    println!("Product not found.");
                }
            }
            Err(e) => {
                eprintln!("Error fetching products: {}", e);
                return;
            }
        }
    }

    if sale_items.is_empty() {
        println!("No items added to sale.");
        return;
    }

    let sale = Sale::new(sale_items);
    
    println!("\nSale Summary:");
    println!("-------------");
    for item in &sale.items {
        if let Ok(Some(product)) = db.get_product(&item.product_id) {
            println!("Product: {}", product.name);
            println!("Quantity: {}", item.quantity);
            println!("Unit Price: ${:.2}", item.unit_price);
            println!("Total: ${:.2}", item.total_price);
            println!("-------------");
        }
    }
    println!("Total Amount: ${:.2}", sale.total_amount);

    if prompt("\nConfirm sale? (y/N): ").to_lowercase() == "y" {
        match sale.validate() {
            Ok(()) => {
                match db.record_sale(&sale) {
                    Ok(()) => println!("\nSale recorded successfully!"),
                    Err(e) => eprintln!("\nError recording sale: {}", e),
                }
            }
            Err(e) => eprintln!("\nValidation error: {}", e),
        }
    } else {
        println!("\nSale cancelled.");
    }

    println!("\nPress Enter to continue...");
    prompt("");
}

fn view_sales(db: &Database) {
    clear_screen();
    display_logo();
    println!("\nSales History");
    println!("-------------");

    match db.get_all_sales() {
        Ok(sales) => {
            if sales.is_empty() {
                println!("No sales recorded.");
            } else {
                for sale in sales {
                    let date: DateTime<Utc> = DateTime::from_timestamp(sale.timestamp, 0).unwrap();
                    println!("\nSale ID: {}", sale.id);
                    println!("Date: {}", date.format("%Y-%m-%d %H:%M:%S"));
                    println!("Items:");
                    for item in sale.items {
                        if let Ok(Some(product)) = db.get_product(&item.product_id) {
                            println!("  - {} x {} (${:.2} each)", product.name, item.quantity, item.unit_price);
                        }
                    }
                    println!("Total Amount: ${:.2}", sale.total_amount);
                    println!("-------------");
                }
            }
        }
        Err(e) => eprintln!("Error fetching sales: {}", e),
    }

    println!("\nPress Enter to continue...");
    prompt("");
}

fn display_main_menu() {
    println!("Main Menu");
    println!("---------");
    println!("1. Product Management");
    println!("2. Sales Management");
    println!("3. Exit");
    println!("\nSelect an option: ");
}

fn display_product_menu() {
    println!("Product Management");
    println!("-----------------");
    println!("1. Add Product");
    println!("2. List Products");
    println!("3. Edit Product");
    println!("4. Delete Product");
    println!("5. Back to Main Menu");
    println!("\nSelect an option: ");
}

fn display_sales_menu() {
    println!("Sales Management");
    println!("---------------");
    println!("1. Record Sale");
    println!("2. View Sales");
    println!("3. Back to Main Menu");
    println!("\nSelect an option: ");
}

fn handle_product_menu(db: &mut Database) {
    loop {
        clear_screen();
        display_logo();
        display_product_menu();

        let choice = prompt("");
        match choice.trim() {
            "1" => add_product(db),
            "2" => list_products(db),
            "3" => edit_product(db),
            "4" => delete_product(db),
            "5" => break,
            _ => {
                println!("Invalid option. Press Enter to continue...");
                prompt("");
            }
        }
    }
}

fn handle_sales_menu(db: &mut Database) {
    loop {
        clear_screen();
        display_logo();
        display_sales_menu();

        let choice = prompt("");
        match choice.trim() {
            "1" => record_sale(db),
            "2" => view_sales(db),
            "3" => break,
            _ => {
                println!("Invalid option. Press Enter to continue...");
                prompt("");
            }
        }
    }
}

fn main() {
    let mut db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };

    loop {
        clear_screen();
        display_logo();
        display_main_menu();

        let choice = prompt("");
        match choice.trim() {
            "1" => handle_product_menu(&mut db),
            "2" => handle_sales_menu(&mut db),
            "3" => {
                println!("\nGoodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Press Enter to continue...");
                prompt("");
            }
        }
    }
}
