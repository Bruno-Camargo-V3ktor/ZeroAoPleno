use rocket::{ launch, routes, get };
use rocket::response::content::RawJson;

#[ get("/") ]
fn index() -> RawJson<&'static str> {
    RawJson(
        r#"
        {
            "menssage": "Bem-Vindo ao Rocket!"
        }
        "#
    )
}


#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
