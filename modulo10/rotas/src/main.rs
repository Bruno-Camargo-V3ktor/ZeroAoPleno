use rocket::{get, routes, launch};

#[ get( "/teste" ) ]
fn teste() -> &'static str {
    "Teste"
}

// Isso e um 'controller' ele receber requisicao e devolver uma resposta
// com as <_> podemos pegar um valor dinamicamente de nossa rota e passar para a funcao controladora
// o nome que esta entre os <...> deve ser o mesmo nome do parametro a qual o valor deve ser atribuido dentro da funcao
#[ get( "/hello/<name>" ) ]
fn hello(name: String) -> String {
    format!( "Ola, seja bem vindo {name}" )
}

#[ get( "/<n>" ) ]
fn number(n: i32) -> String {
    format!( "{n} * 2 = {}", n * 2 )
}

// Rota com Query Params (Parametros de URL) obrigatorios
#[ get( "/search?<query>&<max_results>&<page>" ) ]
fn search(query: String, max_results: i32, page: i32) -> String {
    format!( "Searching for '{query}' (max:{max_results}, page:{page}) " )
}

#[ get( "/filter?<query>&<name>" ) ]
fn filter( query: String, name: Option<String> ) -> String {

    match name {
        Some(t) => format!( "Buscando o {query}, com o nome: {t} " ),
        None => format!( "Buscando o {query}" )
    }

}

#[ launch ]
fn init() -> _ {
    rocket::build().mount("/", routes![teste, hello, number, search, filter])
}
