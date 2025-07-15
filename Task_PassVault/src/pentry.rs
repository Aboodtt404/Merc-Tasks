use rusqlite::{Connection, Result};
use std::io::{self, Write};
// Commented out JSON-related imports
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::fs::OpenOptions;
// use std::io::{BufRead, BufReader};

// #[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        Self { service, username, password }
    }

    // JSON file storage implementation (commented out)
    /*
    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {    
        serde_json::to_string(&self)
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json().expect("Failed to serialize to JSON"));

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json") {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_output.as_bytes()) {
                        eprintln!("Failed to write to file: {}", e);
                    } else {
                        println!("Successfully wrote to file");
                    }
                }
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
    }
    */

    // SQLite implementation
    pub fn save_to_db(&self) -> Result<()> {
        let conn = Connection::open("passwords.db")?;
        
        // Create table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY,
                service TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                UNIQUE(service, username)
            )",
            [],
        )?;

        // Insert new password entry
        conn.execute(
            "INSERT OR REPLACE INTO passwords (service, username, password) VALUES (?1, ?2, ?3)",
            [&self.service, &self.username, &self.password],
        )?;

        Ok(())
    }
}

// JSON file reading implementation (commented out)
/*
pub fn read_password_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = match File::open("passwords.json") {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);
    let mut services = Vec::new();
    let mut seen_entries = std::collections::HashSet::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if json_string.trim().is_empty() {
                continue;
            }
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                // Create a unique key for the service entry
                let entry_key = format!("{}-{}", service_info.service, service_info.username);
                if !seen_entries.contains(&entry_key) {
                    seen_entries.insert(entry_key);
                    services.push(service_info);
                }
            }
        }
    }
    Ok(services)
}
*/

// SQLite implementations
pub fn read_passwords_from_db() -> Result<Vec<ServiceInfo>> {
    let conn = Connection::open("passwords.db")?;
    
    // Create table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            service TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            UNIQUE(service, username)
        )",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT service, username, password FROM passwords")?;
    let password_iter = stmt.query_map([], |row| {
        Ok(ServiceInfo {
            service: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
        })
    })?;

    let mut passwords = Vec::new();
    for password in password_iter {
        passwords.push(password?);
    }

    Ok(passwords)
}

pub fn search_passwords_in_db(search_term: &str) -> Result<Vec<ServiceInfo>> {
    let conn = Connection::open("passwords.db")?;
    let search_pattern = format!("%{}%", search_term.to_lowercase());
    
    let mut stmt = conn.prepare(
        "SELECT service, username, password FROM passwords 
         WHERE LOWER(service) LIKE ?1"
    )?;
    
    let password_iter = stmt.query_map([search_pattern], |row| {
        Ok(ServiceInfo {
            service: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
        })
    })?;

    let mut passwords = Vec::new();
    for password in password_iter {
        passwords.push(password?);
    }

    Ok(passwords)
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| {
            eprintln!("Failed to read line");
            0
        });
    input.trim().to_string()
}
