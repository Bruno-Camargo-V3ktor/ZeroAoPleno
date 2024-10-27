
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CHF,
    AUD,
    CAD,
    BRL,
}

pub struct Account {
    balance: f64,
    currency: Currency,
}

impl Account {

    pub fn new(currency: Currency, balance: f64) -> Self {
        Self{currency, balance}
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance < amount {return false;}
        self.balance -= amount;
        true
    }

    pub fn check_balance(&self) -> f64 {
        self.balance
    }



    pub fn convert_to(&self,  target_currency: Currency ) -> f64 {

        let exchange_rate = {
            match target_currency {
                Currency::BRL => 5.2,
                _ => 1.0
            }
        };

        self.balance * exchange_rate
    }
}