use rocket;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use rocket::{get, routes, Route};

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

#[get("/")]
fn get_all_route() -> Json<Vec<User>> {
    Json(users())
}

pub fn routes() -> Vec<Route> {
    routes![get_all_route]
}

#[cfg(test)]
mod tests {

    #[test]
    fn initial_test() {
        assert_eq!(true, true)
    }
}
