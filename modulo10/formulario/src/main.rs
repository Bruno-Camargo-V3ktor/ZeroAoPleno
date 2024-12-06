use rocket::{ launch, routes, get, post };
use rocket::response::content;
use rocket::http::Status;
use rocket::serde::json::{Json};
use serde::{Deserialize, Serialize};


#[ derive(Serialize, Deserialize) ]
struct FormInput {
    username: String,
    password: String
}

fn validate_input( form_input: &FormInput ) -> Result<(), String> {

    if form_input.username.is_empty() {
        return Err( "Username is required".to_string() )
    }

    if form_input.password.is_empty() {
        return Err( "Password is required".to_string() )
    }

    Ok(())
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
                <form method="post" action="/api/form">

                    <label for = "name">Nome</ label>
                    <input type="text" name="name" />

                    <br />

                    <label for = "password">Senha: </ label>
                    <input type="password" name="password" />

                    <br />

                    <button type = "submit">Enviar</button>

                </form>
            </body>
        </html>
        "#; // r#""# -> Declara uma String literal, alem de pode quebrqa linhas, ele ignora os caracteres especias como \n

    content::RawHtml( html.to_string() )
}

#[ post( "/form", data="<form_input>" ) ]
fn submit_form( form_input: Json<FormInput> ) -> Result< content::RawHtml<String>, (Status, content::RawHtml<String>) > {

    match validate_input( &form_input.0 ) {
        Ok(_) => {
            let msg = format!( "Username: {} \nPassword: {}", form_input.username, form_input.password );

            let html = content::RawHtml(
                format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                        <head>
                            <title>Formulario</title>
                        </head>

                        <body>
                            <h1>Sucesso!</h1>
                            <p>{msg}</p>
                        </body>
                    </html>
                    "#
                )
            );

            Ok( html )
        }

        Err( msg ) => {
            let html = content::RawHtml(
                format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                        <head>
                            <title>Formulario</title>
                        </head>

                        <body>
                            <h1>Error!</h1>
                            <p>{msg}</p>
                            <p> <a href="/api/form">Voltar</a> </p>
                        </body>
                    </html>
                    "#
                )
            );

            Err( (Status::UnprocessableEntity, html) )
        }
    }
}

#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/api/", routes![form, submit_form])
}
