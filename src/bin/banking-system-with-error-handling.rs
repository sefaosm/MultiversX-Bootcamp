// bootcamp-tasks/src/bin/banking-system-with-error-handling.rs
// # Banking System with Error Handling
// This program demonstrates a banking system where users can deposit and withdraw money from their accounts, with error handling for invalid operations and insufficient funds.
// 
// Author: Sefa Osmanoğlu

// Define the Account trait with Result return types
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

#[allow(dead_code)]
// Define the BankAccount struct
struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

// Implement the Account trait for BankAccount
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

// Implementation block for BankAccount constructor
impl BankAccount {
    fn new(account_number: String, holder_name: String, balance: f64) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance
        }
    }
}

fn main() {
    // Create two bank account instances
    let mut account1 = BankAccount::new(
        "1234567890".to_string(),
        "Sefa Osmanoglu".to_string(),
        200.0
    );
    
    let mut account2 = BankAccount::new(
        "0987654321".to_string(),
        "Deniz Cinar".to_string(),
        400.0
    );

    // Test deposit with error handling
    println!("Attempting to deposit into account1...");
    match account1.deposit(1000.0) {
        Ok(()) => println!("Successfully deposited $1000.0"),
        Err(e) => println!("Error during deposit: {}", e),
    }

    // Test deposit with negative amount (should fail)
    println!("\nAttempting to deposit negative amount into account1...");
    match account1.deposit(-50.0) {
        Ok(()) => println!("Successfully deposited $-50.0"),
        Err(e) => println!("Error during deposit: {}", e),
    }

    // Test withdrawal with error handling
    println!("\nAttempting to withdraw from account2...");
    match account2.withdraw(500.0) {
        Ok(()) => println!("Successfully withdrew $500.0"),
        Err(e) => println!("Error during withdrawal: {}", e),
    }

    // Print final balances
    println!("\nFinal Balances:");
    println!("Account 1 ({}) balance: ${:.2}", account1.holder_name, account1.balance());
    println!("Account 2 ({}) balance: ${:.2}", account2.holder_name, account2.balance());
}