use rocket::{launch, routes, get};
use rocket::response::content;

#[ get( "/<msg>/bar" ) ]
fn foo( msg: String ) -> content::RawHtml<String> { content::RawHtml( format!(" <p>Foo {msg} bar!</p> ") ) }

#[ get( "/<_..>" ) ]
fn hey() -> content::RawHtml<String> { content::RawHtml( "Hey, you're here.".to_string() ) }


#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/foo", routes![foo])
        .mount("/", routes![hey])
}
