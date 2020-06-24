#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket_cors;

mod user;

use rocket::http::Method;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

fn main() {
    let (allowed_origins, failed_origins) =
        AllowedOrigins::some(&["http://localhost:3000", "http://localhost:8000"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", StaticFiles::from("client/build"))
        .mount("/users", user::routes())
        .attach(options)
        .launch();
}
