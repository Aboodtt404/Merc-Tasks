mod product;
mod db;
mod sale;
mod purchase;

use std::io::{self, Write};
use chrono::{DateTime, Utc};
use crate::product::Product;
use crate::sale::{Sale, SaleItem};
use crate::db::Database;
use crate::purchase::Purchase;

#[allow(dead_code)]
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn display_logo() {
    println!("╔══════════════════════════════════════════╗");
    println!("║             R u S T O C K                ║");
    println!("║      Inventory Management System         ║");
    println!("╚══════════════════════════════════════════╝\n");
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[allow(dead_code)]
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
    println!("╔══════════════════════════════════════════╗");
    println!("║            CARGO REGISTRY                ║");
    println!("╚══════════════════════════════════════════╝\n");

    match db.get_products() {
        Ok(products) => {
            if products.is_empty() {
                println!("No cargo items in registry.");
            } else {
                for product in &products {
                    println!("┌─ {} ─", product.name);
                    println!("│  ID: {}", product.id);
                    println!("│  Price: ${:.2}", product.price);
                    println!("│  Stock Level: {}", product.quantity);
                    if !product.description.is_empty() {
                        println!("│  Description: {}", product.description);
                    }
                    println!("└──────────────────────────────────────");
                }
                println!("\nTotal Items in Registry: {}", products.len());
            }
        }
        Err(e) => println!("Error fetching cargo items: {}", e),
    }

    prompt("\nPress Enter to continue...");
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

fn delete_product(db: &mut Database) {
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
    println!("╔══════════════════════════════════════════╗");
    println!("║             NEW TRADE OUT                ║");
    println!("╚══════════════════════════════════════════╝\n");

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
    println!("╔══════════════════════════════════════════╗");
    println!("║          TRADING HISTORY                 ║");
    println!("╚══════════════════════════════════════════╝\n");

    match db.get_all_sales() {
        Ok(sales) => {
            if sales.is_empty() {
                println!("No trades recorded yet.");
            } else {
                for (product_name, quantity, total_price, sale_date) in sales {
                    println!("┌─ Trade Details ─");
                    println!("│  Product: {}", product_name);
                    println!("│  Quantity: {}", quantity);
                    println!("│  Total Price: ${:.2}", total_price);
                    println!("│  Date: {}", sale_date);
                    println!("└──────────────────────────────────────\n");
                }
            }
        }
        Err(_e) => {
            println!("No trades recorded yet.");
            println!("Press Enter to continue...");
            prompt("");
            return;
        }
    }

    println!("Press Enter to continue...");
    prompt("");
}

fn display_main_menu() {
    println!("╔══════════════════════════════════════════╗");
    println!("║            RuSTOCK CONSOLE               ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  [1] Cargo Management                    ║");
    println!("║  [2] Trading Operations                  ║");
    println!("║  [3] Supply Chain                        ║");
    println!("║  [4] Reports                             ║");
    println!("║  [5] Exit Terminal                       ║");
    println!("╚══════════════════════════════════════════╝");
    println!("\nEnter your choice (1-5): ");
}

fn display_product_menu() {
    println!("╔══════════════════════════════════════════╗");
    println!("║            CARGO MANAGEMENT              ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  [1] View Cargo Registry                 ║");
    println!("║  [2] Modify Cargo                        ║");
    println!("║  [3] Remove Cargo                        ║");
    println!("║  [4] Return to Console                   ║");
    println!("╚══════════════════════════════════════════╝");
    println!("\nEnter your choice (1-4): ");
}

fn display_sales_menu() {
    println!("╔══════════════════════════════════════════╗");
    println!("║          TRADING OPERATIONS              ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  [1] New Trade Out                       ║");
    println!("║  [2] View Trade History                  ║");
    println!("║  [3] Return to Console                   ║");
    println!("╚══════════════════════════════════════════╝");
    println!("\nEnter your choice (1-3): ");
}

fn display_reports_menu() {
    println!("╔══════════════════════════════════════════╗");
    println!("║               REPORTS                    ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  [1] Inventory Report                    ║");
    println!("║  [2] Sales Report                        ║");
    println!("║  [3] Purchase History Report             ║");
    println!("║  [4] Return to Console                   ║");
    println!("╚══════════════════════════════════════════╝");
    println!("\nEnter your choice (1-4): ");
}

fn display_purchase_menu() {
    println!("╔══════════════════════════════════════════╗");
    println!("║            SUPPLY CHAIN                  ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  [1] New Trade In                        ║");
    println!("║  [2] View Supply History                 ║");
    println!("║  [3] Return to Console                   ║");
    println!("╚══════════════════════════════════════════╝");
    println!("\nEnter your choice (1-3): ");
}

#[allow(dead_code)]
fn display_product_details(product: &Product) {
    println!("╔══════════════════════════════════════════╗");
    println!("║            CARGO DETAILS                 ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  ID: {}", product.id);
    println!("║  Name: {}", product.name);
    println!("║  Description: {}", product.description);
    println!("║  Price: ${:.2}", product.price);
    println!("║  Stock Level: {}", product.quantity);
    println!("╚══════════════════════════════════════════╝");
}

fn handle_product_menu(db: &mut Database) {
    loop {
        clear_screen();
        display_logo();
        display_product_menu();

        let choice = prompt("");
        match choice.trim() {
            "1" => list_products(db),
            "2" => edit_product(db),
            "3" => delete_product(db),
            "4" => break,
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

fn handle_reports_menu(db: &mut Database) {
    loop {
        clear_screen();
        display_logo();
        display_reports_menu();

        let choice = prompt("");
        match choice.trim() {
            "1" => list_products(db),
            "2" => view_sales(db),
            "3" => view_purchases(db),
            "4" => break,
            _ => {
                println!("Invalid option. Press Enter to continue...");
                prompt("");
            }
        }
    }
}

fn record_purchase(db: &mut Database) {
    clear_screen();
    display_logo();
    println!("╔══════════════════════════════════════════╗");
    println!("║             NEW TRADE IN                 ║");
    println!("╚══════════════════════════════════════════╝\n");

    println!("Select operation:");
    println!("1. Purchase existing cargo");
    println!("2. Purchase new cargo");
    println!("3. Return to menu");

    match prompt("\nEnter your choice (1-3): ").trim() {
        "1" => {
            match db.get_products() {
                Ok(products) => {
                    if products.is_empty() {
                        println!("\nNo existing cargo items found.");
                        prompt("\nPress Enter to continue...");
                        return;
                    }

                    println!("\nExisting Cargo Items:");
                    for product in &products {
                        println!("\n┌─ {} ─", product.name);
                        println!("│  ID: {}", product.id);
                        println!("│  Current Price: ${:.2}", product.price);
                        println!("│  Current Stock: {}", product.quantity);
                        println!("└──────────────────────────────────────");
                    }

                    let product_id = prompt("\nEnter Cargo ID: ");
                    let quantity = prompt("Enter Quantity: ").parse::<i32>().unwrap_or(0);
                    let purchase_price = prompt("Enter Purchase Price per Unit: $").parse::<f64>().unwrap_or(0.0);

                    if quantity <= 0 || purchase_price <= 0.0 {
                        println!("\nInvalid quantity or price. Purchase cancelled.");
                        prompt("\nPress Enter to continue...");
                        return;
                    }

                    if let Ok(Some(mut product)) = db.get_product(&product_id) {
                        product.quantity += quantity;
                        if let Err(e) = db.update_product(&product) {
                            println!("\nError updating product quantity: {}", e);
                            prompt("\nPress Enter to continue...");
                            return;
                        }
                    } else {
                        println!("\nProduct with ID {} not found.", product_id);
                        prompt("\nPress Enter to continue...");
                        return;
                    }
                    
                    let total_cost = quantity as f64 * purchase_price;

                    let purchase = Purchase::new(product_id, quantity, purchase_price);

                    match db.record_purchase(&purchase) {
                        Ok(_) => {
                            println!("\nPurchase recorded successfully!");
                            println!("Total Cost: ${:.2}", total_cost);
                        }
                        Err(e) => {
                            println!("\nError recording purchase: {}", e);
                        }
                    }
                }
                Err(e) => println!("Error fetching products: {}", e),
            }
        }
        "2" => {
            println!("\nEnter New Cargo Details:");
            let name = prompt("Name: ");
            let description = prompt("Description: ");
            let selling_price = prompt("Selling Price per Unit: $").parse::<f64>().unwrap_or(0.0);
            let quantity = prompt("Purchase Quantity: ").parse::<i32>().unwrap_or(0);
            let purchase_price = prompt("Purchase Price per Unit: $").parse::<f64>().unwrap_or(0.0);

            if quantity <= 0 || purchase_price <= 0.0 || selling_price <= 0.0 {
                println!("\nInvalid quantity or price. Purchase cancelled.");
                prompt("\nPress Enter to continue...");
                return;
            }

            let mut product = Product {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.clone(),
                description,
                price: selling_price,
                quantity: 0,
                created_at: Utc::now().timestamp(),
                updated_at: Utc::now().timestamp(),
            };

            match db.add_product(&product) {
                Ok(_) => {
                    product.quantity += quantity;
                    if let Err(e) = db.update_product(&product) {
                        println!("\nError updating product quantity: {}", e);
                        prompt("\nPress Enter to continue...");
                        return;
                    }

                    let purchase = Purchase::new(product.id.clone(), quantity, purchase_price);

                    match db.record_purchase(&purchase) {
                        Ok(_) => {
                            println!("\nNew cargo created and purchase recorded successfully!");
                            println!("Total Cost: ${:.2}", purchase.total_cost);
                        }
                        Err(e) => {
                            println!("\nError recording purchase: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("\nError creating new product: {}", e);
                }
            }
        }
        "3" => return,
        _ => {
            println!("\nInvalid option.");
        }
    }

    prompt("\nPress Enter to continue...");
}

fn view_purchases(db: &Database) {
    clear_screen();
    display_logo();
    println!("=== Purchase History ===\n");

    match db.get_all_purchases() {
        Ok(purchases) => {
            if purchases.is_empty() {
                println!("No purchase history available.");
            } else {
                let mut total_cost = 0.0;
                for purchase in purchases {
                    let product_name = db.get_product(&purchase.product_id)
                        .ok()
                        .flatten()
                        .map_or_else(|| "Unknown".to_string(), |p| p.name);

                    println!("Product: {}", product_name);
                    println!("Quantity: {}", purchase.quantity);
                    println!("Purchase Price: ${:.2}/unit", purchase.purchase_price);
                    println!("Total Cost: ${:.2}", purchase.total_cost);
                    let dt = DateTime::<Utc>::from_timestamp(purchase.purchase_date, 0).unwrap();
                    println!("Date: {}", dt.format("%Y-%m-%d %H:%M:%S"));
                    println!("------------------");
                    total_cost += purchase.total_cost;
                }
                println!("\nTotal Purchases Cost: ${:.2}", total_cost);
            }
        }
        Err(e) => {
            println!("Error fetching purchase history: {}", e);
        }
    }

    prompt("\nPress Enter to continue...");
}

fn handle_purchase_menu(db: &mut Database) {
    loop {
        clear_screen();
        display_logo();
        display_purchase_menu();

        let choice = prompt("");
        match choice.trim() {
            "1" => record_purchase(db),
            "2" => view_purchases(db),
            "3" => break,
            _ => {
                println!("Invalid option. Press Enter to continue...");
                prompt("");
            }
        }
    }
}

#[allow(dead_code)]
fn display_error(message: &str) {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║              RUNTIME ERROR               ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  {:<38} ║", message);
    println!("╚══════════════════════════════════════════╝");
}

#[allow(dead_code)]
fn display_success(message: &str) {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║           OPERATION SUCCESS              ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  {:<38} ║", message);
    println!("╚══════════════════════════════════════════╝");
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
            "3" => handle_purchase_menu(&mut db),
            "4" => handle_reports_menu(&mut db),
            "5" => {
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
