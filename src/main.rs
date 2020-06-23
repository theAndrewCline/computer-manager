#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    first_name: String,
    last_name: String,
}

#[get("/")]
fn users() -> Json<User> {
    let first_name = String::from("Andrew");
    let last_name = String::from("Cline");
    let id = 1;

    let user = User {
        first_name,
        last_name,
        id,
    };

    Json(user)
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("client/build"))
        .mount("/users", routes![users])
        .launch();
}
