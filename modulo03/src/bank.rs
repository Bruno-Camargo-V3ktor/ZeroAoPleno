
// Enums
pub enum Currency {
    USD,
    EUR,
    BRA
}

// Traits
pub trait BankAccount {

    fn new(balance: f64, currency: Currency) -> Self;

    fn deposit(&mut self, balance: f64);

    fn withdraw(&mut self, balance: f64) -> bool;

    fn check_balance(&self) -> f64;

}

// Structs
pub struct Account {
    balance: f64,
    currency: Currency
}

// Impls
impl BankAccount for Account {

    fn new(balance: f64, currency: Currency) -> Self { Self { balance, currency } }

    fn deposit(&mut self, balance: f64) {
        if balance < 0.0 { return; }
        self.balance += balance;
    }

    fn withdraw(&mut self, balance: f64) -> bool {
        if self.balance < balance { return false; }
        self.balance -= balance;
        true
    }

    fn check_balance(&self) -> f64 { self.balance }
}
