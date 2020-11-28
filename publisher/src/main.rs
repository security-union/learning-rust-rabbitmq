#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

fn main() {
    rocket().launch();
}

#[get("/")]
fn get() -> String {
    "Hello World".to_string()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/rocket-test",
        routes![get],
    )
}