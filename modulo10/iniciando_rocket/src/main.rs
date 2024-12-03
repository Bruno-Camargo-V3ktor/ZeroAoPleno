//#[ macro_use ] extern crate rocket; // Importando todas as macros do pacote Rocket
use rocket::{get, routes, launch}; // Importando macros em especificos do pacote Rocket

// Adicionando a anotacao "get" esta definindo uma rota do metodo Http GET
#[ get("/") ]
fn index() -> &'static str {
    "Hello, Rocket"
}

// Entrada princiapal de um programa Rocket a partir do 0.5 >
#[ launch ]
fn rocket() -> _ {
    // Criando um build da aplicacao, montando ela na raiz do "servido" e passando uma "lista" de rotas
    rocket::build().mount("/", routes![index])
}

/*
    Em versoes anteriores a 0.5 para a utilizacao das macros do Rocket era nescessario adiciona algumas features na primeira linha:
        - #![ feature(proc_macro_hygiene, decl_macro) ]
            -> Elas habilitam recursos experimentais do compilador rust nightly, nesse caso era obrigatorio habilita
                esses dois recursos, hoje em dia nao e mais obrigatorio, a nao ser em casos muito especificos

    Tambem para iniciamos o projeto precisavamos da funcao main:
    fn main() {
        rocket::ignite().mount("/", routes![index]);
    }

*/
