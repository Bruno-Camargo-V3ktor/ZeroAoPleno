use rocket::{ launch, routes, get };
use rocket::http::Status;


#[ get( "/" ) ]
fn index() -> Status { Status::NotAcceptable }

#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
