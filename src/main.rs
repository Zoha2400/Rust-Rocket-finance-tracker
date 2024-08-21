#[macro_use] extern crate rocket;

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




#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, get_time])
}