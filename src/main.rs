#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
struct Message {
    content: String,
}

#[get("/message")]
fn message() -> Json<Message> {
    Json(Message {
        content: "Hello from Rust!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![message])
}
