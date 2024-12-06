use rocket::{ launch, routes, get };
use rocket::response::content;

#[ get( "/ola/<nome>/<idade>/<legal>" ) ]
fn saudacao(nome: String, idade: u8, legal: bool) -> content::RawHtml<String> {
    let icon = if legal { "💎" } else { "😞" };
    let msg = {
        if legal { format!( "Você é uma pessoa legal de {idade} anos, {nome}!" ) }
        else { format!( "{nome}, precisamos conversar sobre suas atitudes." ) }
    };

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Saudacoes</title>
            </head>

            <body>
                <h1>{icon}</h1>
                <p>{msg}</p>
            </body>
        </html>
        "#
    );

    content::RawHtml( html )
}


#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![saudacao])
}
