mod pentry;

use std::io::Write;
use crate::pentry::{prompt, ServiceInfo};

fn clr() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = std::io::stdout().flush();
}

fn display_ascii() {
    let ascii = r#"
    ____                __     __           _ _   
    |  _ \ __ _ ___ ___\ \   / /_ _ _   _| | |_ 
    | |_) / _` / __/ __\ \ / / _` | | | | | __|
    |  __/ (_| \__ \__ \\ V / (_| | |_| | | |_ 
    |_|   \__,_|___/___/ \_/ \__,_|\__,_|_|\__|
    "#;
    println!("{}", ascii);
}

fn main() {
    clr();
    display_ascii();
    
    loop {
        println!("\nPassword manager menu: ");
        println!("1. Add Entry ");
        println!("2. List Entry ");
        println!("3. Search Entry ");
        println!("4. Quit ");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                display_ascii();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: ")
                );
                match entry.save_to_db() {
                    Ok(_) => println!("Entry added successfully"),
                    Err(e) => eprintln!("Error saving entry: {}", e),
                }
            }
            "2" => {
                clr();
                display_ascii();
                match pentry::read_passwords_from_db() {
                    Ok(passwords) => {
                        if passwords.is_empty() {
                            println!("No entries found.");
                        } else {
                            println!("\nStored Entries:");
                            println!("--------------------------------");
                            for item in passwords {
                                println!("Service: {}", item.service);
                                println!("Username: {}", item.username);
                                println!("Password: {}", item.password);
                                println!("--------------------------------");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading passwords: {}", err);
                    }
                }
            }
            "3" => {
                clr();
                display_ascii();
                let search = prompt("Search: ");
                match pentry::search_passwords_in_db(&search) {
                    Ok(passwords) => {
                        if passwords.is_empty() {
                            println!("No matching entries found.");
                        } else {
                            for item in passwords {
                                println!("--------------------------------");
                                println!("Service: {}", item.service);
                                println!("Username: {}", item.username);
                                println!("Password: {}", item.password);
                                println!("--------------------------------");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error searching passwords: {}", err);
                    }
                }
            }
            "4" => {
                clr();
                display_ascii();
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}