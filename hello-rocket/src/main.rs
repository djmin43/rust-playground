#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world! hello rust"
}

#[get("/test")]
fn test() -> &'static str {
    "test"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world])
        .mount("/", routes![world, test])
}