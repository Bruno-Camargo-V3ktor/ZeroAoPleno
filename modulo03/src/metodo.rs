
pub struct People {
    pub nome: String,
    sobrenome: String
}


impl People {
    pub fn new(nome: String, sobrenome: String) -> Self {  Self { nome, sobrenome }  }

    pub fn qual_nome(&self) {
        println!("Meu nome eh {}", self.nome);
    }

    pub fn nome_completo(&self) {
        println!("Meu completo eh {} {}", self.nome, self.sobrenome);
    }

}

pub fn metodo_teste() {
   println!( "Metodo Teste" );
}
