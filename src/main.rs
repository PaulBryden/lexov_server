use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;
use lexov_core::data_structures::{Position};
use serde_json;
use rand::prelude::*;
/// A WebSocket echo server
fn main () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                println!("here!");
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
                println!("here! 2");
                    match websocket.write_message(Message::Text(serde_json::to_string(&Position { x: rand::random::<f64>()*100.0, y: rand::random::<f64>()*100.0, z: rand::random::<f64>()*100.0 }).unwrap()))
                    {
                        Ok(success) => println!("success"),
                        Err(err) => eprintln!("error: {}", err),
                    };
                
            }
        });
    }
}