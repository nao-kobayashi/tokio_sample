use tokio::prelude::*;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

use futures::try_ready;

struct EchoServer {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl EchoServer {
    fn new(socket: UdpSocket) -> Self {
        EchoServer { socket, buf: vec![0; 1024] }
    }
}

impl Future for EchoServer {
    type Item = ();
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        let _ = try_ready!(self.socket.poll_recv_from(&mut self.buf));
        println!("connect");
        Ok(Async::Ready(()))
    }
}

pub fn execute() {
    let addr = "127.0.0.1:9000".to_string().parse::<SocketAddr>().unwrap();
    let socket = UdpSocket::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let server = EchoServer::new(socket);
    tokio::run(server.map_err(|_| ()));
}

/*
impl Future for EchoServer {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        match self.socket.poll_recv_from(&mut self.buf) {
            Ok(Async::Ready((_size, _peer))) => {
                println!("recv");
                Ok(Async::Ready(()))
            },
            Ok(Async::NotReady) => {
                println!("not ready");
                Ok(Async::NotReady)
            },
            Err(_) => Err(()),
        }
    }
}

pub fn execute() {
    let addr = "127.0.0.1:9000".to_string().parse::<SocketAddr>().unwrap();
    let socket = UdpSocket::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let server = EchoServer::new(socket);
    tokio::run(server);
}
*/