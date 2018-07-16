extern crate actix;
extern crate actors_tutorial;

use actix::prelude::*;
use actors_tutorial::*;

fn main() {
    let sys = actix::System::new("test");

    let counter: Addr<Syn, _> = Arbiter::start(|_| CounterActor::new());
    let counter_addr_copy = counter.clone();

    let result = counter.send(PlusOne)
        .and_then(move |count| {
            println!("Count: {}", count);
            counter_addr_copy.send(PlusOne)
        })
        .map(|count| {
            println!("Count: {}", count);
            Arbiter::system().do_send(actix::msgs::SystemExit(0));
        })
        .map_err(|error| {
            println!("An error occured: {}", error);
        });

    Arbiter::handle().spawn(result);

    sys.run();
}
