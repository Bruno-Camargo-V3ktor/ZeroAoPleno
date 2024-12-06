use rocket::{ launch, routes, get };
use rocket::response::content;
//use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};


#[ derive(Serialize, Deserialize) ]
struct FormInput {
    username: String,
    password: String
}

#[ get( "/form" ) ]
fn form() -> content::RawHtml<String> {
    let html =
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Formulario</title>
            </head>

            <body>
                <h1>Exemplo de Formulario</h1>
                <form method="post" action="/form">

                    <label for = "name">Nome</ label>
                    <input type="text" name="name" />

                    <br />

                    <label for = "e-mail">E-mail</ label>
                    <input type="email" name="e-mail" />

                    <br />

                    <button type = "submit">Enviar</button>

                </form>
            </body>
        </html>
        "#; // r#""# -> Declara uma String literal, alem de pode quebrqa linhas, ele ignora os caracteres especias como \n

    content::RawHtml( html.to_string() )
}



#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![form])
}
