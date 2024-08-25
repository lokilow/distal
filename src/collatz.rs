use actix::prelude::*;

#[derive(Debug)]
pub struct CollatzActor;
#[derive(Debug)]
pub struct CollatzWorker;

impl Actor for CollatzActor {
    type Context = Context<Self>;
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }

    fn stopping(&mut self, _ctx: &mut Context<Self>) -> Running {
        Running::Continue
    }
}

impl Actor for CollatzWorker {
    type Context = SyncContext<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Calculate(pub usize);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DoCalculate(pub Addr<CollatzActor>, pub usize);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Done {
    arg: usize,
    iterations: usize,
}

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

impl Handler<Calculate> for CollatzActor {
    type Result = ();

    fn handle(&mut self, msg: Calculate, ctx: &mut Context<Self>) -> Self::Result {
        let addr = ctx.address();
        let Calculate(val) = msg;
        let worker = SyncArbiter::start(1, || CollatzWorker);
        worker.do_send(DoCalculate(addr, val));

        println!("Started worker to run the collatz algorithm on {}", val)
    }
}
impl Handler<Done> for CollatzActor {
    type Result = ();

    fn handle(&mut self, msg: Done, _ctx: &mut Context<Self>) -> Self::Result {
        let Done { arg, iterations } = msg;
        println!(
            "The collatz algortithm for {} took {} iterations to complete",
            arg, iterations
        )
    }
}

impl Handler<DoCalculate> for CollatzWorker {
    type Result = ();

    fn handle(&mut self, msg: DoCalculate, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let DoCalculate(from, initial_val) = msg;

        let iterations = collatz(initial_val);
        let response = Done {
            arg: initial_val,
            iterations,
        };
        from.do_send(response);
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
