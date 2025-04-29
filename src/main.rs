// Import Rocket macros globally
#[macro_use] extern crate rocket;

// This handler responds to GET requests at "/"
#[get("/")]  // <-- Match browser requests too
fn index() -> &'static str {
    "Home"  // HTML content
}

// This handler responds to GET requests at "/about"
#[get("/about")]
fn about() -> &'static str {
    "About"
}

// This handler responds to GET requests at "/contact"
#[get("/contact")]
fn contact() -> &'static str {
    "Contact"
}

// This is the Rocket launch function
// It builds a Rocket instance and mounts all routes under "/"
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about, contact])
        // This means:
        //   GET /          → index()
        //   GET /about     → about()
        //   GET /contact   → contact()
}
