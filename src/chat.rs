extern crate futures;
extern crate tokio_core;
extern crate chat;
extern crate tokio_io;

use std::env;
use std::str;
use std::net::SocketAddr;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use chat::ChatServer;

fn main() {
    let mut lp = Core::new().unwrap();
    let handle = lp.handle();

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();
    println!("Listening on {}", addr);

    let chat_server = ChatServer::new(listener);
    lp.run(chat_server).unwrap();
}
