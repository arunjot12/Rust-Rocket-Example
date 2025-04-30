#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::serde::{Serialize};

// Define a simple struct to send as JSON
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Info {
    title: String,
    message: String,
}

// Return a JSON object at GET /data
#[get("/data")]
fn data() -> Json<Info> {
    Json(Info {
        title: "Welcome to Rocket!".into(),
        message: "This content is served from Rust backend.".into(),
    })
}

// Launch the Rocket app and serve static + routes
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static"))) // Serves index.html
        .mount("/api", routes![data])                      // API route
}
