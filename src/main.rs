use actix::prelude::*;
mod collatz;


#[actix::main]
async fn main() {
    // start new actor
    let addr = collatz::CollatzActor { count: 10 }.start();

    // send message and get future for result
    let res = addr.send(collatz::Ping(10)).await;

    // handle() returns tokio handle
    println!("RESULTING COUNT: {}", res.unwrap());

    // stop system and exit
    System::current().stop();
}
