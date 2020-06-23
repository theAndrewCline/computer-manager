#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::serve::StaticFiles;

use user::User;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("client/build"))
        .mount("/users", routes![users])
        .launch();
}
