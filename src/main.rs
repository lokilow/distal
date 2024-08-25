use actix::prelude::*;
// use std::env;
mod collatz;

// #[actix::main]
fn main() {
    let system = System::new();
    let execution = async {
        let addr = collatz::CollatzActor.start();

        let arr: [usize; 100] = (1..=100)
            .collect::<Vec<_>>()
            .try_into()
            .expect("wrong size iterator");

        let _ = for i in arr {
            addr.do_send(collatz::Calculate(i))
        };
    };
    Arbiter::current().spawn(execution);

    // stop system and exit
    // Don't call, I want all the calculations to finish
    // System::current().stop();
    let _ = system.run();
}
