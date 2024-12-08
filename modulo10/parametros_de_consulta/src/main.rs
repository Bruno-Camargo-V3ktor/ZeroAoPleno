use rocket::{ launch, build, routes, get };
use rocket::response::content::RawHtml;
use rocket::response::status::BadRequest;


#[ get( "/<param>?<a>&<b>&<c>" ) ]
fn index( param: usize, a: Option<String>, b: Option<u32>, c: Option<bool> ) -> Result<RawHtml<String>, BadRequest<String>> {

    let a = { a.unwrap_or( "default_value".to_string() ) };
    let b = { b.unwrap_or( 0 ) };
    let c = { c.unwrap_or( false ) };

    Ok( RawHtml( format!( r#"<h1>Received param: {param}, a: {a}, b: {b}, c: {c}</h1>"# ) ) )
}

#[ launch ]
fn start() -> _ {
    build()
        .mount("/", routes![index])
}
