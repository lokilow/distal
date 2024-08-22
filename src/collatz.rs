use actix::prelude::*;
pub struct CollatzActor {
    pub count: usize,
}

impl Actor for CollatzActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

impl Handler<Ping> for CollatzActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}
