#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate amiquip;

use rocket::{
    Rocket,
    State,
};

use rocket_contrib::json::Json;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use amiquip::{Connection, Exchange, Publish};

type RockState<'a, T> = State<'a, Arc<Mutex<T>>>;

fn main() {
    rocket().launch();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub msg: String,
    pub queue_id: String
}

#[post(
    "/message",
    format = "application/json",
    data = "<message>"
)]
fn post(message: Json<Message>, state: RockState<Connection>) -> Json<Message> {
    let msg_clone = message.clone();
    let arc = state.inner();
    let arc = arc.clone();
    let connection = arc.lock().unwrap();
    let channel = connection.open_channel(None).expect("Unable to open channel");
    let exchange = Exchange::direct(&channel);
    exchange.publish(Publish::new(msg_clone.msg.as_bytes(), msg_clone.queue_id)).expect("Unable to publish to exchange");
    message
}

fn rocket() -> rocket::Rocket {
    let mut connection = Connection::insecure_open("amqp://rabbitmq:rabbitmq@rabbit:5672").expect("Unable to reach rabbitmq");
    rocket::ignite().mount(
        "/rocket-test",
        routes![post],
    ).manage(Arc::new(Mutex::new(connection)))
}
