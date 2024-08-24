use actix::prelude::*;
use std::env;
mod collatz;

#[actix::main]
async fn main() {
    // Initial value for collatz algorithm
    const DEFAULT_VAL: usize = 10;
    let val = env::args()
        .nth(1)
        .expect(&format!("{DEFAULT_VAL}"))
        .parse::<usize>()
        .unwrap();

    // start new actor
    let addr = collatz::CollatzActor.start();

    // send message and get future for result
    let res = addr.send(collatz::Run { val }).await;

    // handle() returns tokio handle
    println!("RESULTING COUNT: {}", res.unwrap());

    // stop system and exit
    System::current().stop();
}
