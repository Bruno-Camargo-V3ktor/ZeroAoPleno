pub struct Repositorio<T> {
    dados: Vec<T>,
}

impl<T> Repositorio<T> {
    pub fn new() -> Self {
        Repositorio { dados: vec![] }
    }

    pub fn adicionar(&mut self, item: T) {
        self.dados.push(item);
    }
}

impl Repositorio<i32> {
    pub fn total(&self) -> i32 {
        self.dados.iter().sum()
    }
}

impl Repositorio<f64> {
    pub fn produto(&self) -> f64 {
        self.dados.iter().product()
    }
}
