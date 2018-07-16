extern crate actix;
use actix::prelude::*;

// `PlusOne` message implementation

pub struct PlusOne;

impl Message for PlusOne {
    type Result = u32;
}

// `CounterActor` implementation

pub struct CounterActor {
    count: u32,
}

impl CounterActor {
    pub fn new() -> CounterActor {
        CounterActor { count: 0 }
    }
}

impl Actor for CounterActor {
    type Context = Context<Self>;
}

impl Handler<PlusOne> for CounterActor {
    type Result = u32;

    fn handle(&mut self, _msg: PlusOne, _ctx: &mut Context<Self>) -> u32 {
        self.count += 1;
        self.count
    }
}
