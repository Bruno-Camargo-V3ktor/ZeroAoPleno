use std::fmt::{write, Display, Formatter};

// Struct
pub struct Doenca {
    nome: String,
    sintomas: Vec<String>,
    causa: String,
    tratamento: String,
}

// Implements
impl Doenca {

    pub fn new(nome: &str, sintomas: Vec<String>, causa: &str, tratamento: &str ) -> Self {

        Self {
            nome : nome.to_string(),
            sintomas :sintomas,
            causa : causa.to_string(),
            tratamento : tratamento.to_string(),
        }

    }

    pub fn getNome(&self) -> String { self.nome.clone() }
    pub fn getSintomas(&self) -> Vec<String> { self.sintomas.clone() }
    pub fn getCausa(&self) -> String { self.causa.clone() }
    pub fn getTratamento(&self) -> String { self.tratamento.clone() }

    pub fn info(&self) {

    }

}

impl Display for Doenca {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f,
        " Informacoes da Doenca: {}
         -- Sintomas: {:?}
         -- Causa: {}
         -- Tratamento: {}", self.nome, self.sintomas, self.causa, self.tratamento)

    }
}
