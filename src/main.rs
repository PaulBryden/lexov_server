use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use lexov_core::data_structures::{Position};
/// A WebSocket echo server
fn main () {
    let _pos_struct = Position { x: 1.0, y: 2.0, z: 3.0 };
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}