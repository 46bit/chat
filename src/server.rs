use futures::{Future, Sink, Stream, Poll, Async, AsyncSink};
use tokio_core::net::{TcpListener, Incoming};
use tokio_io::AsyncRead;
use comms::{Client, Room};
use super::*;

pub struct ChatServer {
    id_counter: u64,
    incoming: Incoming,
    clients: Room<u64, StringTransport>,
}

impl ChatServer {
    pub fn new(listener: TcpListener) -> ChatServer {
        ChatServer {
            id_counter: 0,
            incoming: listener.incoming(),
            clients: Room::default(),
        }
    }

    fn start_broadcast(&mut self, from_id: u64, msg: String) {
        // Broadcast this received message to all other clients.
        for to_id in self.clients.ids() {
            if to_id == from_id {
                continue;
            }
            match self.clients.start_send((to_id, msg.clone())) {
                Ok(AsyncSink::Ready) => {}
                Ok(AsyncSink::NotReady((to_id, msg))) => {
                    // No buffering is performed inside `Room` or `Client`.
                    println!("Dropped message. to_id={:?} msg={:?}", to_id, msg);
                }
                Err(e) => {
                    // Room's impl on Sink returns non-fatal errors. These indicate
                    // a client disconnection, or an unknown client ID.
                    println!("Client Room Sink start_send error:\n    {:?}", e);
                }
            }
        }
    }
}

impl Future for ChatServer {
    type Item = ();
    type Error = String;

    fn poll(&mut self) -> Poll<(), String> {
        println!("ChatServer wakes up with connected IDs {:?}",
                 self.clients.ids());

        // Accept new clients.
        match self.incoming.poll() {
            Ok(Async::NotReady) => {}
            Ok(Async::Ready(Some((socket, addr)))) => {
                // Assign incrementing IDs to clients.
                let client_id = self.id_counter;
                self.id_counter += 1;
                println!("New client addr={:?} id={:?}", addr, client_id);
                // Add new client to the Room.
                let client = Client::new(client_id, socket.framed(StringCodec));
                self.clients.insert(client);
            }
            Ok(Async::Ready(None)) => return Err("Incoming tcp client Stream died.".to_string()),
            Err(e) => return Err(format!("Incoming tcp client Stream error: {:?}", e)),
        }

        // We have to read from the Stream until it is empty.
        loop {
            match self.clients.poll() {
                Ok(Async::NotReady) => break,
                Ok(Async::Ready(Some((id, msg)))) => {
                    println!("Broadcasting message from {:?}:\n    {:?}",
                             id.clone(),
                             msg.clone());
                    self.start_broadcast(id, msg);
                }
                Ok(Async::Ready(None)) => return Err("Client Room Stream poll died.".to_string()),
                Err(e) => {
                    println!("Client Room Stream error:\n    {:?}", e);
                }
            }
        }

        // Make progress sending to all clients.
        match self.clients.poll_complete() {
            Ok(Async::NotReady) | Ok(Async::Ready(())) => {}
            Err(e) => {
                println!("Client Room Sink poll_complete error:\n    {:?}", e);
            }
        }

        println!("ChatServer quietens down");
        // Always NotReady, unless an unrecoverable error happens.
        Ok(Async::NotReady)
    }
}
