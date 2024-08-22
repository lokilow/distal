use actix::prelude::*;
use std::env;
mod collatz;

#[actix::main]
async fn main() {
    let initial_value = env::args().nth(1).expect("10").parse::<usize>().unwrap();
    // start new actor
    let addr = collatz::CollatzActor {
        count: initial_value,
    }
    .start();

    // send message and get future for result
    let res = addr.send(collatz::Ping(10)).await;

    // handle() returns tokio handle
    println!("RESULTING COUNT: {}", res.unwrap());

    // stop system and exit
    System::current().stop();
}
