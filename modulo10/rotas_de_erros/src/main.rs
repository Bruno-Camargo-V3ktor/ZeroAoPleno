use rocket::{launch, routes, get, catchers, catch, Request };


#[ get("/msg") ]
fn msg() -> String { "Hello, mensagem de teste!!!".to_string() }

#[ catch(404) ] // Usando a macro catch e passando o ocodigo do error que queros tratar dentro dessa funcao
fn not_found(req: &Request) -> String { format!( "Pagina nao encontrada :( \nURL: '{}' Invalida", req.uri().path() ) }

/*
    As funcoes marcadas com a macro 'catch' podem receber como parametro (nao obrigatorio) um ponteiro para a Strcut Request,
    Que representa as informacoes da requisicao feita do usuario ao servidor
*/

#[ launch ]
fn start() -> _ {

    rocket::build()
        .mount("/", routes![msg])
        .register("/", catchers![not_found]) // Registrando rotas para manipulacao de erros como 404, 401, 500 e etc....

}
