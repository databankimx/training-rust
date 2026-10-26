mod bank {
    pub struct Account {
        balance: i64,
    }

    impl Account {
        pub fn new() -> Self {
            Account { balance: 0 }
        }

        pub fn deposit(&mut self, amount: i64) {
            self.balance += amount;
        }

        pub fn withdraw(&mut self, amount: i64) -> Result<(), String> {
            if amount > self.balance {
                Err(format!("cannot withdraw {amount}, balance is {}", self.balance))
            } else {
                self.balance -= amount;
                Ok(())
            }
        }

        pub fn balance(&self) -> i64 {
            self.balance
        }
    }
}

use bank::Account;

fn main() {
    let mut acct = Account::new();
    acct.deposit(100);

    match acct.withdraw(30) {
        Ok(()) => println!("withdrew 30"),
        Err(e) => println!("{e}"),
    }

    match acct.withdraw(500) {
        Ok(()) => println!("withdrew 500"),
        Err(e) => println!("{e}"),
    }

    println!("balance: {}", acct.balance());
}
