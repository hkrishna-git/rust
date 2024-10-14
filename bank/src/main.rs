#[derive(Debug)]
struct Account  {
    id: u32,
    balance: i32,
    holder: String
}

#[derive(Debug)]
struct Bank {
    accounts:Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank {accounts: vec![]}
    }

    fn addAccount(&mut self, account:Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account |account.balance).sum()
    }
}

impl Account {
    fn new(id:u32, holder:String) -> Self {
        Account {
            id:id, 
            balance:0, 
            holder:holder
        }
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }
 
    fn deposit(&mut self, amount:i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdrawl(&mut self, amount:i32) -> i32 {
        self.balance -= amount;
        self.balance
    }
}

fn printAccount(account:Account) {
    println!("{:#?}", account);
}
 
fn main() {
    let mut bank = Bank::new();
    let mut account1 = Account::new(1, String::from("hk1"));
    let mut account2 = Account::new(2, String::from("hk2"));

    account1.deposit(500);
    account1.withdrawl(200);

    account2.deposit(1000);
    account2.withdrawl(250);

    println!("{}", account1.summary());
    println!("{}", account2.summary());

    bank.addAccount(account1);
    bank.addAccount(account2);

    println!("{:#?}", bank.total_balance());
}
