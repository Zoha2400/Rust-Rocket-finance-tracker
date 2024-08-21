mod db;

#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::{Rocket, Build};

#[derive(FromForm)]
struct RegisterForm {
    name: String,
    email: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hello(name: String) -> String {
    format!("Hello {}", name)
}

#[get("/time/<seconds>")]
fn get_time(seconds: u64) -> String {
    format!("Current time: {} seconds", seconds)
}

#[post("/register", data = "<form>")]
fn register(form: Form<RegisterForm>) -> &'static str {
    let RegisterForm { name, email } = form.into_inner();
    let result = db::register_user(&name, &email);
    if result {
        "Registered successfully"
    } else {
        "Not registered"
    }
}

// Uncomment and fix the get_user route if needed
// #[get("/users/<id>")]
// fn get_user(id: u32) -> String {
//     db::get_user(id).unwrap_or_else(|e| format!("Error fetching user: {}", e))
// }

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, hello, get_time, register])
}
