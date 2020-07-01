#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket_cors;

use rocket::http::Method;
use rocket::{get, routes, State};
use rocket_contrib::serve::StaticFiles;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod user;

#[get("/")]
fn hit_count(count: State<u32>) -> String {
    format!("count {}", count.inner())
}

fn main() {
    rocket::ignite()
        .manage(0)
        .mount("/", StaticFiles::from("client/build"))
        .mount("/count", routes![hit_count])
        .mount("/users", user::routes())
        .attach(cors())
        .launch();
}

fn cors() -> rocket_cors::Cors {
    let (allowed_origins, failed_origins) =
        AllowedOrigins::some(&["http://localhost:3000", "http://localhost:8000"]);
    assert!(failed_origins.is_empty());

    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}
