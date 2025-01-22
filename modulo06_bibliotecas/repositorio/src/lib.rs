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

impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Default> Repositorio<T> {
    pub fn total(&self) -> T {
        self.dados
            .iter()
            .copied()
            .fold(T::default(), |acc, x| acc + x)
    }

    pub fn produto(&self) -> T {
        let mut init = 0;
        self.dados.iter().copied().fold(T::default(), |acc, x| {
            if init == 0 {
                init = 1;
                acc + x
            } else {
                acc * x
            }
        })
    }
}

/*impl Repositorio<i32> {
    pub fn total(&self) -> i32 {
        self.dados.iter().sum()
    }
}

impl Repositorio<f64> {
    pub fn produto(&self) -> f64 {
        self.dados.iter().product()
    }
}
*/
