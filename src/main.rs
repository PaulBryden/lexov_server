use lexov_core::data_structures::{Player, Position};
use mut_static::MutStatic;
use rand::prelude::*;
use serde_json;
use std::io::Write;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;
pub mod PlayerTracker;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;
/// Lexov Simple Web Server
///
///

//Quick and dirty way to make a thread safe singleton
lazy_static! {
    static ref DATA: MutStatic<PlayerTracker::PlayerTracker::PlayerListContainer> =
        MutStatic::new();
}


//Everything runs very simply and succinctly within main();
//Threads are forked for every connection.
//These threads record the current player position and transmit the rest of the player positions.
//When the thread disconnects, we remove the player from the player tracker.
fn main() {
    DATA.set(PlayerTracker::PlayerTracker::PlayerListContainer::new())
        .unwrap(); //Initialize thread safe singleton
        
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let uuidVar: u64 = rand::random::<u64>();
            loop {
                let mut player: Player = Player {
                    uuid: uuidVar,
                    position: Position {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                };
                let msg = match websocket.read_message() {
                    Ok(message) => message,
                    Err(error) => {
                        DATA.write().unwrap().remove(uuidVar);
                        let mut output = String::new();
                        panic!("Client Error: {:?}", error);
                    }
                };
                if msg.is_text() {
                    let pos: lexov_core::data_structures::Position =
                        serde_json::from_str(&msg.into_text().unwrap()).unwrap();
                    player.position = pos;
                    DATA.write().unwrap().update(player);
                    match websocket.write_message(Message::Text(
                        DATA.read().unwrap().getDataExcludeUUID(uuidVar),
                    )) {
                        Ok(message) => message,
                        Err(error) => {
                            DATA.write().unwrap().remove(uuidVar);
                            panic!("Client Error: {:?}", error);
                        }
                    }
                }
            }
        });
    }
}
