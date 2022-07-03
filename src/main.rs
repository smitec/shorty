#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::response::Redirect;

use std::collections::HashMap;

pub mod models;
pub mod schema;

#[get("/s/<short>")]
fn index(short: &str) -> Redirect {
    let mut short_long = HashMap::new();

    short_long.insert("project".to_string(), "https://www.google.com".to_string());

    if short_long.contains_key(short) {
        let url = short_long.get(short).unwrap();
        Redirect::to(url.clone())
    } else {
        Redirect::to(uri!("/lost"))
    }
}

#[get("/lost")]
fn lost() -> String {
    "Nothing Here".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![lost])
}
