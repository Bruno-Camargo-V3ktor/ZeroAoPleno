use rocket::{ launch, routes, get };
use rocket::response::content::RawHtml;

#[ get( "/user/<id>" )  ]
fn get_user( id: usize ) -> RawHtml<String> {
    RawHtml( format!( "Usuario com id: {id} || tipo usize" ) )
}

// Usando o parametro Rank, dizemos ao Rocket que quando tive rotas se colidindo, se a padrao (default) falha por algum motivo, ele tentara
    //executa as proximas conforma o valor do rank, ate encontra uma que der certo, ou se tenta todas e falhar
#[ get( "/user/<id>", rank = 2 ) ]
fn get_user_isize( id: isize ) -> RawHtml<String> {
    RawHtml( format!( "Usuario com id: {id} || tipo isize" ) )
}

#[ get( "/user/<id>", rank = 3 ) ]
fn get_user_str( id: String ) -> RawHtml<String> {
    RawHtml( format!( "Usuario com id: {id} || tipo Str" ) )
}


#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![get_user, get_user_isize, get_user_str])
}
