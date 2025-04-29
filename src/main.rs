#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello in rust, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}