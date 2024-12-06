use rocket::{ launch, routes };
use rocket::response::content;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};


#[ derive(Serialize, Deserialize) ]
struct FormInput {
    username: String,
    password: String
}




#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![])
}
