
use tokio::prelude::*;

struct MyFuture;

impl Future for MyFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        println!("poll called");
        Ok(Async::Ready(()))
    }
}

pub fn execute() {
    tokio::run(MyFuture{});
    println!("finished");
}