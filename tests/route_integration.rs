use rocket::local::Client;

// initial setup for integration tests

#[test]
fn it_works() {
    let rocket = rocket::ignite();
    Client::new(rocket).expect("valid rocket instance");
}
