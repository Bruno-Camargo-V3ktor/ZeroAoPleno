use std::fmt::{Display, Formatter};

// Structs
pub struct Imobiliaria {
    nome: String,
    endereco: String,
    imoveis: Vec<Imovel>
}

pub struct Imovel {
    endereco: String,
    preco: f32,
    num_quartos: u8,
    num_banheiros: u8,
    metragem: u16
}

// Impls
impl Imovel {
    pub fn new( endereco: &str, preco: f32, num_quartos: u8, num_banheiros: u8, metragem: u16 ) -> Self {
        Self { endereco: endereco.to_string(), preco, num_quartos, num_banheiros, metragem }
    }
}

impl Display for Imovel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "Imovel - ( Endereco: '{}', Preco: {}, Numero de Quartos: {}, Numero de Banheiros: {}, Metragem: {} )",
         self.endereco, self.preco, self.num_quartos, self.num_banheiros, self.metragem
        )
    }

}


impl Imobiliaria {

    pub fn new( nome: &str, endereco: &str ) -> Self {
        Self{ nome: nome.to_string(), endereco: endereco.to_string(), imoveis: vec![] }
    }

    pub fn add_imovel(&mut self, endereco: &str, preco: f32, num_quartos: u8, num_banheiros: u8, metragem: u16 ) {
        self.imoveis.push( Imovel{ endereco: endereco.to_string(), preco, num_quartos, num_banheiros, metragem } )
    }

    pub fn show_imoves(&self) {
        for movel in &self.imoveis { println!("{}", movel); }
    }

}
