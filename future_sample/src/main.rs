extern crate futures;
extern crate tokio;

mod first_future;
mod second_future;
mod echo_server;

fn main() {
    //first_future::execute();
    // second_future::execute();
    echo_server::execute();
}
