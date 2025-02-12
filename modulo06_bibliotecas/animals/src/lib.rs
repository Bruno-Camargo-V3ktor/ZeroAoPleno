// Traits
pub trait Animal {
    fn fazer_barulho(&self);
}

// Structs
pub struct Gato;
pub struct Cachorro;

// Impls
impl Animal for Gato {
    fn fazer_barulho(&self) {
        println!("O Gato faz Miuauuuu");
    }
}

impl Animal for Cachorro {
    fn fazer_barulho(&self) {
        println!("O Cachorro faz Au au au");
    }
}
