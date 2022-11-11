#[macro_use] extern crate rocket;

use rocket::time::Duration;
use rocket::tokio::time::sleep;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world! hello rust"
}

#[get("/test")]
fn test() -> &'static str {
    "test"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world])
        .mount("/", routes![world, test])
}