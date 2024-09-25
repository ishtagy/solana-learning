trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be greater than zero".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be greater than zero".to_string());
        } else if amount > self.balance {
            Err("Not enough balance to withdraw".to_string())
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
    let mut first_account = BankAccount {
        account_number: 1111,
        holder_name: String::from("Bob"),
        balance: 1000.0,
    };
    let mut second_account = BankAccount {
        account_number: 2222,
        holder_name: String::from("Alice"),
        balance: 500.0,
    };

    first_account.deposit(200.0);
    second_account.withdraw(300.0);

    match first_account.deposit(0.0) {
        Ok(()) => println!("Deposit successful"),
        Err(e) => println!("Deposit failed: {}", e),
    }

    match second_account.withdraw(7000.0) {
        Ok(()) => println!("Withdraw successfull"),
        Err(e) => println!("Withdraw failed: {}", e),
    }

    println!(
        "{} balance: {}",
        first_account.holder_name,
        first_account.balance()
    );
    println!(
        "{} balance: {}",
        second_account.holder_name,
        second_account.balance()
    );
}
