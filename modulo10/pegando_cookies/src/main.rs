use rocket::{ launch, routes, get };
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::RawHtml;

// CookieJar eh um Struct que existe durante o contexto de uma requisao Http
    // Com ele podemos pegar valores de cookies ja presenteno navegador
    // Ou registra cookies no navegador do usuario
    // Tambem possuindo a possibilidade de Cookies secreto, onde o usario nao ver

#[ get("/") ]
fn print_cookie( cookie: &CookieJar ) -> RawHtml<String> {

    let message = {
        match cookie.get( "message" ) {
            Some( c ) => c.value(),
            None => ""
        }
    };

    RawHtml( format!( "Message: {message}" ) )
}


#[ get("/<message>") ]
fn register_cookie( message: String, cookies: &CookieJar ) -> RawHtml<String>{

    let cookie = Cookie::build( ("message", message) ).path("/");
    cookies.add( cookie );

    RawHtml( format!( "Cookie Registrado com Sucesso" ) )
}

#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount( "/", routes![print_cookie, register_cookie] )
}
