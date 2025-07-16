use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Manager {
    pub id: String,
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub created_at: i64,
    pub is_active: bool,
}

impl Manager {
    pub fn new(username: String, password: String, full_name: String) -> Self {
        Manager {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            password,
            full_name,
            created_at: Utc::now().timestamp(),
            is_active: true,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.username.trim().is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        
        if self.username.len() < 3 {
            return Err("Username must be at least 3 characters".to_string());
        }
        
        if self.password.trim().is_empty() {
            return Err("Password cannot be empty".to_string());
        }
        
        if self.password.len() < 4 {
            return Err("Password must be at least 4 characters".to_string());
        }
        
        if self.full_name.trim().is_empty() {
            return Err("Full name cannot be empty".to_string());
        }
        
        Ok(())
    }
}

pub struct AuthService;

impl AuthService {
    pub fn authenticate(username: &str, password: &str, managers: &[Manager]) -> Option<Manager> {
        managers
            .iter()
            .find(|manager| {
                manager.username == username 
                    && manager.password == password 
                    && manager.is_active
            })
            .cloned()
    }
    
    pub fn is_valid_credentials(username: &str, password: &str) -> bool {
        !username.trim().is_empty() && !password.trim().is_empty()
    }
} 