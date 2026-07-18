fn main(){
    let mut account: BankAccount = BankAccount{
        owner : "Alice".to_string(),
        balance: 150.55,
    };
    //immutable borrow to check the balance
    account.check_balance();
    //mutable borrow to withdraw money
    account.withdraw(30.0);
    account.check_balance();
}

struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self){
        println!("Accounnt owned by {} has a balance of {}", self.owner, self.balance);
    }
}
