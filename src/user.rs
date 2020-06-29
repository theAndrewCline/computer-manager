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
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn get_all_route_test() {
        let rocket = rocket::ignite().mount("/", routes());
        let client = Client::new(rocket).expect("valid rocket instance");

        let mut response = client.get("/").dispatch();

        let expected = serde_json::to_string(&users()).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(response.body_string(), Some(expected));
    }
}
