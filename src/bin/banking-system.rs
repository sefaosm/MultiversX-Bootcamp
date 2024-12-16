// bootcamp-tasks/src/bin/banking-system.rs
// # Banking System
// This program demonstrates a banking system where users can deposit and withdraw money from their accounts.
// Apart from the task, added a verification step before deposit and withdrawal
//
// Author: Sefa Osmanoğlu

trait Account {
    fn deposit(&mut self, amount: u32) -> bool;
    fn withdraw(&mut self, amount: u32) -> bool;
    fn balance(&self) -> u32;
    // This is the additional verification method i added
    fn verify_account(&self, account_number: u32) -> bool;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: u32
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: u32) -> bool {
        // Optional verification step before deposit
        if !self.verify_account(self.account_number) {
            println!("Account verification failed. Deposit rejected.");
            return false;
        }
        
        self.balance += amount;
        true
    }

    fn withdraw(&mut self, amount: u32) -> bool {
        // Optional verification step before withdrawal
        if !self.verify_account(self.account_number) {
            println!("Account verification failed. Withdrawal rejected.");
            return false;
        }
        
        if amount <= self.balance {
            self.balance -= amount;
            true
        } else {
            println!("Insufficient funds");
            false
        }
    }

    fn balance(&self) -> u32 {
        self.balance
    }

    fn verify_account(&self, account_number: u32) -> bool {
        // Simple verification logic
        // In a real system, this would involve more complex checks
        // Here, we'll use a basic validation method
        let valid_ranges = [
            (1000..2000),   // Range for standard accounts
            (2000..3000),   // Range for premium accounts
            (3000..4000)    // Range for business accounts
        ];

        valid_ranges.iter().any(|range| range.contains(&account_number))
    }
}

fn main() {
    // Create two bank accounts
    let mut account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("Sefa Osmanoglu"),
        balance: 0
    };

    let mut account2 = BankAccount {
        account_number: 4500,  // Invalid account number
        holder_name: String::from("Deniz Cinar"),
        balance: 800
    };

    // Demonstrate verification
    println!("Account 1 Verification: {}", account1.verify_account(account1.account_number));
    println!("Account 2 Verification: {}", account2.verify_account(account2.account_number));

    // Deposit and withdraw with verification
    if account1.deposit(300) {
        println!("{}'s Deposit successful. New Balance: {}", account1.holder_name, account1.balance());
    }

    if account2.withdraw(200) {
        println!("{}'s Withdrawal successful. New Balance: {}", account2.holder_name, account2.balance());
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub(crate) fn test_verify_account() {
        let account = BankAccount {
            account_number: 1001,
            holder_name: String::from("Sefa Osmanoglu"),
            balance: 0
        };

        assert!(account.verify_account(account.account_number));
    }

    #[test]
    pub(crate) fn test_verify_account_invalid() {
        let account = BankAccount {
            account_number: 4500,  // Invalid account number
            holder_name: String::from("Deniz Cinar"),
            balance: 800
        };

        assert!(!account.verify_account(account.account_number));
    }

    #[test]
    pub(crate) fn test_deposit() {
        let mut account = BankAccount {
            account_number: 1001,
            holder_name: String::from("Sefa Osmanoglu"),
            balance: 0
        };

        assert!(account.deposit(300));
        assert_eq!(account.balance(), 300);
    }

    #[test]
    pub(crate) fn test_withdraw() {
        let mut account = BankAccount {
            account_number: 1001,
            holder_name: String::from("Sefa Osmanoglu"),
            balance: 800
        };

        assert!(account.withdraw(200));
        assert_eq!(account.balance(), 600);
    }

    #[test]
    pub(crate) fn test_insufficient_funds() {
        let mut account = BankAccount {
            account_number: 1001,
            holder_name: String::from("Sefa Osmanoglu"),
            balance: 200
        };

        assert!(!account.withdraw(300));
        assert_eq!(account.balance(), 200);
    }
}