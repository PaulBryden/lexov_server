use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;
use lexov_core::data_structures::{Position, Player};
use serde_json;
use rand::prelude::*;
use mut_static::MutStatic;
pub mod PlayerTracker;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;
/// A WebSocket echo server
/// 
/// 
lazy_static! {
    static ref DATA: MutStatic<PlayerTracker::PlayerTracker::PlayerListContainer> =  MutStatic::new();
}
fn main () {
    DATA.set(PlayerTracker::PlayerTracker::PlayerListContainer::new()).unwrap();
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let mut uuidVar: u64 = rand::random::<u64>();
            loop {
                let mut player: Player = Player {uuid: uuidVar, position:  Position{x:0.0, y:0.0, z:0.0}};
                println!("here!");
                let msg = websocket.read_message().unwrap();
                // We do not want to send back ping/pong messages.
                if  msg.is_text() {
                    let pos: lexov_core::data_structures::Position = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
                    player.position = pos;
                    let mut mut_handle = DATA.write().unwrap();
                    mut_handle.update(player);
                    websocket.write_message(Message::Text(mut_handle.getData())).unwrap();
                }
            }
        });
    }
}