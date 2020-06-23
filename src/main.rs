#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::serve::StaticFiles;
#[macro_use]
extern crate rocket;
mod user;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("client/build"))
        .mount("/users", routes![user::get_all_route])
        .launch();
}
