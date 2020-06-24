#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket_cors;

use rocket::http::Method;
use rocket_contrib::serve::StaticFiles;

mod user;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("client/build"))
        .mount("/users", user::routes())
        .launch();
}
