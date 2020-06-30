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

#[get("/<id>")]
fn get_by_id(id: u32) -> Json<Option<User>> {
    let u = users().into_iter().find(|user| user.id == id);

    Json(u)
}

pub fn routes() -> Vec<Route> {
    routes![get_all_route, get_by_id]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn get_all_users() {
        let rocket = rocket::ignite().mount("/", routes());
        let client = Client::new(rocket).expect("valid rocket instance");

        let mut response = client.get("/").dispatch();

        let expected = serde_json::to_string(&users()).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(response.body_string(), Some(expected));
    }


    #[test]
    fn get_by_id() {
        let rocket = rocket::ignite().mount("/", routes());
        let client = Client::new(rocket).expect("valid rocket instance");

        let mut response = client.get("/1").dispatch();


        let andrew = User {
            first_name: String::from("Andrew"),
            last_name: String::from("Cline"),
            id: 1,
        };

        let expected = serde_json::to_string(&andrew).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(response.body_string(), Some(expected));
    }


    #[test]
    fn get_user_with_bad_id() {
        let rocket = rocket::ignite().mount("/", routes());
        let client = Client::new(rocket).expect("valid rocket instance");

        let mut response = client.get("/4").dispatch();

        let u: Option<User> = None;

        let expected = serde_json::to_string(&u).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(response.body_string(), Some(expected));
    }
}
