#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use diesel::Connection;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};

use diesel::sqlite::SqliteConnection;

use std::env;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;

#[derive(Deserialize)]
struct NewLink<'r> {
    short: &'r str,
    long: &'r str,
    secret: &'r str,
}

#[derive(Deserialize)]
struct RemoveLink<'r> {
    short: &'r str,
    secret: &'r str,
}

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqliteConnection::establish(&database_url).expect("Cannot connect to database")
}

#[get("/s/<short_code>")]
fn index(short_code: &str) -> Redirect {
    use schema::links::dsl::*;

    let connection = establish_connection();
    let results = links
        .filter(short.eq(short_code))
        .limit(1)
        .load::<Link>(&connection)
        .expect("Error Loading Links");

    if results.len() == 1 {
        let link = results.get(0).unwrap();

        diesel::update(links.find(link.id))
            .set(hits.eq(link.hits + 1))
            .execute(&connection)
            .expect("Could not update hit count");

        Redirect::to(link.long.to_string())
    } else {
        Redirect::to(uri!("/lost"))
    }
}

#[get("/stats")]
fn stats() -> String {
    use schema::links::dsl::*;

    let connection = establish_connection();
    let results = links
        .load::<Link>(&connection)
        .expect("Error Loading Links");

    let mut result: String = "".to_string();

    for link in results {
        result.push_str(format!("{} -> {} : {} hits\n", link.short, link.long, link.hits).as_str());
    }

    result
}

#[get("/lost")]
fn lost() -> String {
    "Nothing Here".to_string()
}

#[post("/add", format = "json", data = "<newlink>")]
fn add(newlink: Json<NewLink<'_>>) -> &str {
    use schema::links;

    let secret = env::var("ADD_SECRET").expect("ADD_SECRET not set");
    let connection = establish_connection();

    if secret == newlink.0.secret {
        let new_link = AddLink {
            short: newlink.0.short,
            long: newlink.0.long,
        };

        diesel::insert_into(links::table)
            .values(&new_link)
            .execute(&connection)
            .expect("Can't Add Link");

        "OK"
    } else {
        "NOPE"
    }
}

#[post("/del", format = "json", data = "<dellink>")]
fn del(dellink: Json<RemoveLink<'_>>) -> &str {
    use schema::links::dsl::*;

    let secret = env::var("ADD_SECRET").expect("ADD_SECRET not set");
    let connection = establish_connection();

    if secret == dellink.0.secret {
        diesel::delete(links.filter(short.eq(dellink.0.short)))
            .execute(&connection)
            .expect("Can't Remove Link");

        "OK"
    } else {
        "NOPE"
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![lost])
        .mount("/", routes![add])
        .mount("/", routes![del])
        .mount("/", routes![stats])
}
