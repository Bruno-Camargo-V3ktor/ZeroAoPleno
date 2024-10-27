
pub struct Coin {
    value: f64,
}

impl Coin {
    pub fn new(value: f64) -> Coin { Coin { value } }
    pub fn get_value(&self) -> f64 { self.value }
    pub fn set_value(&mut self, value: f64) { self.value = value }
}