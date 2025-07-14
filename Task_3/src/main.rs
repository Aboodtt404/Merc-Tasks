trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

impl BankAccount {
    fn new(account_number: &str, holder_name: &str, initial_balance: f64) -> Self {
        BankAccount {
            account_number: account_number.to_string(),
            holder_name: holder_name.to_string(),
            balance: initial_balance,
        }
    }
}

fn main() {
    let mut user1 = BankAccount::new("ACC001", "User_1", 1000.0);
    let mut user2 = BankAccount::new("ACC002", "User_2", 500.0);

    match user1.deposit(500.0) {
        Ok(_) => println!("Deposit successful!"),
        Err(e) => println!("Deposit failed: {}", e),
    }

    match user2.withdraw(1000.0) {
        Ok(_) => println!("Withdraw successful!"),
        Err(e) => println!("Withdraw failed: {}", e),
    }

    println!("{} Balance: {}", user1.holder_name, user1.balance());
    println!("{} Balance: {}", user2.holder_name, user2.balance());
}
