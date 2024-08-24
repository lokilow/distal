use actix::prelude::*;

#[derive(Debug)]
pub struct CollatzActor;

impl Actor for CollatzActor {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
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

    fn handle(&mut self, msg: Run, ctx: &mut Context<Self>) -> Self::Result {
        dbg!(self);
        dbg!(&msg);
        dbg!(ctx);

        let Run(val) = msg;

        collatz(val)
    }
}

fn collatz(val: usize) -> usize {
    do_collatz(val, 0)
}

fn do_collatz(val: usize, iterations: usize) -> usize {
    if val == 1 {
        iterations
    } else if val % 2 == 0 {
        let next_val = val / 2;
        do_collatz(next_val, iterations + 1)
    } else {
        let next_val = 3 * val + 1;
        do_collatz(next_val, iterations + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(27), 111);
    }
}
