use rocket::{get, post, delete, put, routes, launch};


#[ post("/users/<name>") ]
fn create_user(name: String) {
    println!( "Usuario com o nome: '{name}' Criado ✅" )
}


#[ get("/users/<id>") ]
fn get_user( id: u32 ) -> String {
    format!( "Retornando o usuario com o ID: {id}" )
}


#[ get("/users?<query>&<page>") ]
fn search_users(query: String, page: Option<u32>) -> String {
    match page {
        Some(p) => format!("Buscando usuario com a consulta: '{query}'. Na pagina: {p}"),
        None => format!("Buscando usuario com a consulta: '{query}'.")
    }
}


#[ put("/users/<id>?<name>") ]
fn update_user(id: u32, name: String) {
    println!( "Usuario com o ID: '{id}' Atualizado ✅" );
    println!( "Novo nome eh {name}" );
}


#[ delete("/users/<id>") ]
fn delete_user(id: u32) {
    println!( "Usuario com o ID: '{id}' Deletado ✅" )
}


#[ launch ]
fn start() -> _ { rocket::build().mount("/", routes![create_user, get_user, search_users, update_user, delete_user]) }
