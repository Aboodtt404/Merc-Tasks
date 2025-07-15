mod product;
mod db;

use std::io::{self, Write};
use product::Product;
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

fn main() {
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error initializing database: {}", e);
            return;
        }
    };

    loop {
        clear_screen();
        display_logo();
        
        println!("\nMain Menu");
        println!("---------");
        println!("1. Add Product");
        println!("2. List Products");
        println!("3. Edit Product");
        println!("4. Delete Product");
        println!("5. Exit");

        match prompt("\nSelect an option: ").as_str() {
            "1" => add_product(&db),
            "2" => list_products(&db),
            "3" => edit_product(&db),
            "4" => delete_product(&db),
            "5" => {
                println!("\nGoodbye!");
                break;
            }
            _ => {
                println!("\nInvalid option. Press Enter to continue...");
                prompt("");
            }
        }
    }
}
