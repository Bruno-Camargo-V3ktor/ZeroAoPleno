use rocket::{ launch, routes, post };
use rocket::response::status;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rand::{thread_rng, Rng};


#[ derive(Serialize, Deserialize) ]
struct User {
    id: Option<usize>,
    name: String,
    age: u8,
    email: String,
}


#[ post("/user", format = "application/json", data = "<user>") ]
fn create_user( user: Option< Json<User> > ) -> Result<Json<User>, status::BadRequest<String>> {
    let mut rng = thread_rng();

    let mut user = {
        match user {
            Some( v ) => v.into_inner(),
            None => return Err( status::BadRequest( "Bad Request".to_string() ) )
        }
    };

    user.id = Some( rng.gen_range( 0 .. 100 ) );
    Ok( Json( user ) )
}

#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![create_user])
}
