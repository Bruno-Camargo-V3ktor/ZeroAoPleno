use rocket::{ launch, routes, get, post };
use rocket::http::{Cookie, CookieJar};
use rocket::response::{ Flash, Redirect, status };
use rocket::response::content::RawHtml;


#[ get("/user_id") ]
fn user_id( cookies: &CookieJar ) -> Result< RawHtml<String>, status::NotFound< RawHtml<String> > > {
    match cookies.get_private("user_id") {
        Some(c) => Ok( RawHtml( format!( "User ID: {}", c.value() ) ) ),
        None => Err( status::NotFound(  RawHtml("User not login".to_string()) ) )
    }
}

#[ post("/logout") ]
fn logout( cookies: &CookieJar ) -> Flash<Redirect> {
    cookies.remove_private( "user_id" );
    Flash::success(Redirect::to("/"), "Sucessfully Logout")
}

#[ post("/login", data = "<user_id>") ]
fn login( user_id: String, cookies: &CookieJar ) -> Flash<Redirect> {
    cookies.add_private( Cookie::build(("user_id", user_id)) );
    Flash::success(Redirect::to("/"), "Sucessfully Login")
}

#[ get("/") ]
fn index() -> RawHtml<String> {
    RawHtml(
        format!(
            r#"
            <head>
                <title>Welcome</title>
            </head>

            <body>
                <h1>Welcome Page!</h1>
            </body>
            "#
        )
    )
}

#[ launch ]
fn start() -> _ {
    rocket::build()
        .mount("/", routes![index, user_id, logout, login])
}
