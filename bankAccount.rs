// Define the Account trait
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Implement the BankAccount struct
struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: &str, holder_name: &str) -> BankAccount {
        BankAccount {
            account_number: account_number.to_string(),
            holder_name: holder_name.to_string(),
            balance: 0.0,
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Deposit amount must be greater than zero.".to_string())
        } else {
            self.balance += amount;
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be greater than zero.".to_string())
        } else if amount > self.balance {
            Err("Insufficient funds.".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two bank accounts
    let mut account1 = BankAccount::new("123456", "John Doe");
    let mut account2 = BankAccount::new("987654", "Jane Smith");

    // Deposit money into account1 and handle potential errors
    match account1.deposit(500.0) {
        Ok(()) => println!("Deposit successful. New balance: ${}", account1.balance()),
        Err(e) => println!("Error during deposit: {}", e),
    }

    // Withdraw money from account2 and handle potential errors
    match account2.withdraw(100.0) {
        Ok(()) => println!("Withdrawal successful. New balance: ${}", account2.balance()),
        Err(e) => println!("Error during withdrawal: {}", e),
    }

    // Deposit money into account2 and handle potential errors
    match account2.deposit(200.0) {
        Ok(()) => println!("Deposit successful. New balance: ${}", account2.balance()),
        Err(e) => println!("Error during deposit: {}", e),
    }

    // Withdraw money from account1 and handle potential errors
    match account1.withdraw(1000.0) {
        Ok(()) => println!("Withdrawal successful. New balance: ${}", account1.balance()),
        Err(e) => println!("Error during withdrawal: {}", e),
    }

    // Print the final balances of both accounts
    println!("\nFinal balances:");
    println!("Account 1 ({}) balance: ${}", account1.account_number, account1.balance());
    println!("Account 2 ({}) balance: ${}", account2.account_number, account2.balance());
}
