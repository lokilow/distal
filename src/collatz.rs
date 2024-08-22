use actix::prelude::*;
pub struct CollatzActor;

impl Actor for CollatzActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Run(pub usize);

//
// pub trait Handler<M>
// where
//     Self: Actor,
//     M: Message,
// {
//     type Result: MessageResponse<Self, M>;
//
//     // Required method
//     fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result;
// }
//
//
// pub trait Message {
//     type Result: 'static;
// }
//
// type Result: 'static
// The type of value that this message will resolved with if it is successful.
//

impl Handler<Run> for CollatzActor {
    type Result = usize;

    fn handle(&mut self, msg: Run, _ctx: &mut Context<Self>) -> Self::Result {
        let val: usize = msg.0;
        val

        // collatz(val)
    }
}

// I need to learn rust types real quick
// fn collatz(val)
// fn do_collatz(val, iterations)
