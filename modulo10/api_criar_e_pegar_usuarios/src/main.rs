use std::sync::Mutex;

use rocket::{ launch, routes, get, post };
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::content::RawJson;


#[ derive( Serialize, Deserialize, Clone ) ]
struct User {
    id: Option<usize>,
    name: String,
    age: u8,
    email: String
}

struct UserStore {
    users: Mutex< Vec<User> > //Protegendo o vetor com Mutex para seguranca entre as Theards
}


#[ post("/user", format = "application/json", data = "<user>") ]
fn new_user( user_store: &rocket::State<UserStore>, user: Option<Json<User>> ) -> Result< RawJson<String>, BadRequest< RawJson<String>> > {

    let user = match user {
        Some(u) => u,
        None => {
            return Err( BadRequest( RawJson(
                    format!(
                        r#"
                        {{
                            "err": "Paramets Invalids"
                        }}
                        "#
                    )
                ) ) )
        }
    };

    let mut users = user_store.users.lock().unwrap();
    let mut new_user = user.into_inner();

    new_user.id = Some( users.len() );
    users.push( new_user.clone() );

    Ok( RawJson(
        format!(
            r#"
            {{
                "name": "{}",
                "age": {},
                "email": "{}",
            }}
            "#,
            new_user.name, new_user.age, new_user.email
        )
    ) )
}



#[ get( "/user/<id>", format = "application/json" ) ]
fn get_user_by_id( user_store: &rocket::State< UserStore >, id: usize ) -> Result< Json<User>, NotFound< RawJson<String> > > {

    let users = user_store.users.lock().unwrap();

    match users.get( id ) {
        Some( user ) => Ok( Json( user.clone() ) ),
        None => Err( NotFound( RawJson(
                    format!(
                    r#"
                    {{
                        "err": "User with ID: '{id}' not found"
                    }}
                    "#
                    )
                )))
    }

}

#[ launch ]
fn start() -> _ {

    rocket::build()
        .mount("/", routes![new_user, get_user_by_id])
        .manage( UserStore{ users: Mutex::new( vec![] ) } )

}
