use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    first_name: String,
    last_name: String,
}

#[get("/")]
pub fn get_all_route() -> Json<User> {
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
