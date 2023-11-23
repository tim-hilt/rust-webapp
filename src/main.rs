#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"<html>
    <head>
        <title>Test</title>
    </head>
    <body>
            <h1>Oh, hello there!</h1>
    </body>
    </html>"#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

