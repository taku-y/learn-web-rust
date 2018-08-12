extern crate tungstenite;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

fn main() {
    let server = TcpListener::bind("0.0.0.0:9001").unwrap();
    for stream in server.incoming() {
        println!("Message came");
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            websocket.write_message(777 as u32).unwrap();
            loop {
                println!("In loop");
                let msg = websocket.read_message().unwrap();
                println!("Received: {}", msg);
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
