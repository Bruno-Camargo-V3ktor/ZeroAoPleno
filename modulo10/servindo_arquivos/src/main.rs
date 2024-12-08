use rocket::{ launch };
use rocket::fs::FileServer;


#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/public", FileServer::from("static_files")) // Montando um Roteador
}
