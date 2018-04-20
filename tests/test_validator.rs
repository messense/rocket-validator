#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_lenient_json;
extern crate rocket_validator;
#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;

use rocket::local::Client;
use rocket::http::{Status, ContentType};
use rocket_lenient_json::Json;
use rocket_validator::Validation;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct SignupData {
    #[validate(email)]
    email: String,
    #[validate(length(min = "8"))]
    password: String,
}

#[post("/signup", data = "<data>")]
fn signup(data: Validation<Json<SignupData>>) {
    println!("{:?}", data);
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![signup])
}

#[test]
fn test_validator_failure() {
    let client = Client::new(rocket()).unwrap();

    let res = client.post("/signup")
        .header(ContentType::JSON)
        .body(r#"{ "email": "Hello, world!", "password": "1234qwer" }"#)
        .dispatch();
    assert_eq!(res.status(), Status::BadRequest);
}

#[test]
fn test_validator_success() {
    let client = Client::new(rocket()).unwrap();

    let res = client.post("/signup")
        .header(ContentType::JSON)
        .body(r#"{ "email": "hello@example.com", "password": "1234qwer" }"#)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}
