use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    first_name: String,
    last_name: String,
}

fn users() -> Vec<User> {
    let andrew = User {
        first_name: String::from("Andrew"),
        last_name: String::from("Cline"),
        id: 1,
    };

    let kristin = User {
        first_name: String::from("Kristin"),
        last_name: String::from("Cline"),
        id: 2,
    };

    vec![andrew, kristin]
}

fn user_by_id(id: u32) -> Option<&'static User> {
    users().filter(|user| user.id == id)
}

#[get("/")]
pub fn get_all_route() -> Json<Vec<User>> {
    Json(users())
}
