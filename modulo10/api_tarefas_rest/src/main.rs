use std::sync::Mutex;
use rocket::{ launch, routes, get, post, put, delete };
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::response::status::{BadRequest, NotFound, Custom};
use rocket::http::Status;


// Structs
struct ListTodo {
    todos: Mutex< Vec<Todo> >
}


#[ derive( Deserialize, Serialize, Clone ) ]
struct Todo {
    id: Option<usize>,
    description: String,
    complete: bool
}


#[ derive( Deserialize, Serialize ) ]
struct Error {
    err: String,
    status: u16
}


// Impls
impl Error {
    fn bad_request_json( err: String ) -> BadRequest< Json<Error> > {
        BadRequest( Json( Error{ err, status: 400 } ) )
    }

    fn not_found_json( err: String ) -> NotFound< Json<Error> > {
        NotFound( Json( Error{ err, status: 404 } ) )
    }

    fn custom_status_json( status: Status, err: String ) -> Custom< Json<Error> > {
        Custom( status, Json( Error{ err, status: status.code } ) )
    }

}


// Endpoints
#[ post("/todo", format = "application/json", data = "<todo>") ]
fn create_todo( list: &rocket::State<ListTodo>, todo: Option< Json<Todo> > ) -> Result< Json<Todo>, BadRequest<Json<Error> > > {

    let mut new_todo = {
        match todo {
            Some(t) => t.into_inner(),
            None => return Err( Error::bad_request_json( "Parameters Invalids.".to_string() ) )
        }
    };

    let mut todos = list.todos.lock().unwrap();
    new_todo.id = Some( todos.len() );

    todos.push( new_todo.clone() );

    Ok( Json( new_todo ) )
}


#[ get("/todo") ]
fn get_list( list: &rocket::State<ListTodo> ) -> Json< Vec< Todo > > {
    let todos = list.todos.lock().unwrap();
    Json( todos.clone() )
}


#[ get("/todo/<id>") ]
fn get_todo( list: &rocket::State<ListTodo>, id: usize ) -> Result< Json<Todo>, NotFound<Json<Error> > > {
    let todos = list.todos.lock().unwrap();

    match todos.get(id) {
        Some( t ) => Ok( Json( t.clone() ) ),
        None => Err( Error::not_found_json( format!("Todo by id: '{id}' not found") ) )
    }
}


#[ delete("/todo/<id>") ]
fn delete_todo( list: &rocket::State<ListTodo>, id: usize ) -> Result< (), NotFound<Json<Error> > > {
    let mut todos = list.todos.lock().unwrap();

    for i in 0 .. todos.len() {
        if i == id {
            todos.remove( id );
            return Ok( () )
        }
    }

    Err( Error::not_found_json( format!("Todo by id: '{id}' not found") ) )
}


#[ put("/todo/<id>", format = "application/json", data = "<todo>") ]
fn update_todo( list: &rocket::State<ListTodo>, id: usize, todo: Option< Json<Todo> > ) -> Result< Json<Todo>, Custom<Json<Error> > > {
    let new_todo = {
        match todo {
            Some(t) => t.into_inner(),
            None => return Err( Error::custom_status_json(Status::BadRequest, "Parameters Invalids.".to_string() ) )
        }
    };

    let mut todos = list.todos.lock().unwrap();
    match todos.get_mut(id) {
        Some( t ) => {
            t.description = new_todo.description;
            t.complete = new_todo.complete;
            Ok( Json( t.clone() ) )
        },
        None => Err( Error::custom_status_json(Status::NotFound, format!("Todo by id: '{id}' not found") ) )
    }
}


// Start Server
#[ launch ]
fn start() -> _ {

    rocket::build()
        .mount("/", routes![create_todo, get_list, get_todo, delete_todo, update_todo])
        .manage( ListTodo{ todos: Mutex::new( vec![] ) } )

}
