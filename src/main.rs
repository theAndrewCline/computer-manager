#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::serve::StaticFiles;

struct User {
    id: u32,
    first_name: String,
    last_name: String,
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn users() -> &'static User {
    let first_name = String::from("Andrew");
    let last_name = String::from("Cline");
    let id = 1;

    &User {
        first_name,
        last_name,
        id,
    }
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("client/build"))
        .mount("/users", routes![users])
        .launch();
}
