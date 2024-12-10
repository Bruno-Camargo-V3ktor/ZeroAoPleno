use rocket::{ launch, routes, post };
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::response::status::BadRequest;

#[ derive(Serialize, Deserialize, Clone) ]
struct Todo {
    description: String,
    complete: bool
}

#[ derive(Serialize, Deserialize, Clone) ]
struct Err {
    err: String,
    status: u16
}

impl Err {
    fn new_badrequest_json( err: String ) -> BadRequest< Json<Self> > {
        BadRequest( Json( Self { err, status: 400 } ))
    }
}

#[ post("/todo", format = "application/json", data = "<todo>") ]
fn create_todo( todo: Option<Json<Todo>>  ) -> Result< Json<Todo>, BadRequest< Json<Err> > > {

    match todo {
        Some(t) => Ok( t ),
        None => Err( Err::new_badrequest_json( "Parametrs in Request Invalid".to_string() ) )
    }

}


#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![create_todo])
}
