extern crate failure;
extern crate tungstenite;
extern crate yew;
extern crate ui;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use yew::format::{Text, Json};
use ui::WsResponse;

fn main() {
    let server = TcpListener::bind("0.0.0.0:9001").unwrap();
    for stream in server.incoming() {
        println!("Message came");
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                println!("Reveived: {:?}", msg);
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
                let msg = WsResponse{ value: 333 };
                let msg: Text = Json(&msg).into();
                let msg = tungstenite::protocol::Message::Text(msg.unwrap());
                println!("Prepared: {:?}", msg);
                websocket.write_message(msg).unwrap();
            }
        });
    }
}
