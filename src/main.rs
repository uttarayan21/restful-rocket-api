#[macro_use]
extern crate rocket;

#[get("/?<name>")]
pub fn index(name: &str) -> String {
    format!("Happy Birthday, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
