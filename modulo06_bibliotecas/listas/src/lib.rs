pub struct Lista<T> {
    elementos: Vec<T>,
}

impl<T> Lista<T> {
    pub fn nova() -> Self {
        Self {
            elementos: Vec::new(),
        }
    }

    pub fn adicionar(&mut self, elemento: T) {
        self.elementos.push(elemento);
    }

    pub fn remove(&mut self, indice: usize) -> Option<T> {
        if self.elementos.len() > indice {
            Some(self.elementos.remove(indice))
        } else {
            None
        }
    }

    pub fn buscar(&self, elemento: &T) -> Option<usize>
    where
        T: PartialOrd,
    {
        self.elementos.iter().position(|e| e == elemento)
    }

    pub fn ordenar(&mut self)
    where
        T: Ord,
    {
        self.elementos.sort();
    }
}
