trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Not enough balance to withdraw");
        } else {
            self.balance -= amount;
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
