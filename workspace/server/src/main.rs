#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![feature(proc_macro_non_items)]

extern crate bincode;
extern crate maud;
extern crate rocket;
extern crate rocket_contrib;
extern crate sled;
extern crate tempdir;
extern crate failure;
extern crate tungstenite;
extern crate yew;
extern crate ui;
extern crate wdview_msg;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io;
use std::thread;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use maud::{html, Markup};
use rocket::response::NamedFile;
use rocket::State;
use sled::Tree;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use yew::format::{Text, Json};
use wdview_msg::{WsMessage, Data, Command, Vector, Body, PlotParamForVector};

fn all_routes() -> Vec<rocket::Route> {
    routes![
        index,
        static_file,
        ugly_hack,
    ]
}

/// This is the entry point for our yew client side app.
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
//fn index(db: State<Arc<sled::Tree>>) -> Markup {
//    // maud macro
//    html! {
//        link rel="stylesheet" href="static/styles.css" {}
//        body {}
//        // yew-generated javascript attaches to <body>
//        script src=("static/ui.js") {}
//    }
//}

/// Serve static assets from the "static" folder.
#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

// TODO: remove this when we figure out how to change the native Rust
// WebAssembly's generated JavaScript code to point at "static/" prefix.
#[get("/ui.wasm")]
fn ugly_hack() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/ui.wasm")).ok()
}

fn start_web_server() {
    let path = String::from("data.db");
    let conf = sled::ConfigBuilder::new().path(path).build();
    let tree = Tree::start(conf).unwrap();
    let db_arc = Arc::new(tree);
    let routes = all_routes();
    rocket::ignite().mount("/", routes).manage(db_arc).launch();
}

fn start_websocket_server() {
    let server = TcpListener::bind("0.0.0.0:9001").unwrap();
    for stream in server.incoming() {
        println!("Message came");
        spawn (move || {
            // Handshake
            let mut websocket = accept(stream.unwrap()).unwrap();
            println!("websocket object was created");

            // Send a wdview message
            let msg1 = WsMessage::Data(Data {
                name: "3-dim vector".to_string(),
                body: Body::Vector(Vector {
                    name_row: None,
                    data: vec![4., 2., 8.],
                }),
            });
            let msg2 = PlotParamForVector {
                data_name: "3-dim vector".to_string(),
                area_name: "plot_area".to_string(),
            }.into_wsmsg();

            let msg1 = tungstenite::protocol::Message::Text(serde_json::to_string(&msg1).unwrap());
            let msg2 = tungstenite::protocol::Message::Text(serde_json::to_string(&msg2).unwrap());
            println!("Following messages will be sent for debug");
            println!("{:?}", &msg1);
            println!("{:?}", &msg2);
            websocket.write_message(msg1).unwrap();
            websocket.write_message(msg2).unwrap();

            // Websocket vent loop
            loop {
                let msg = websocket.read_message().unwrap();
                println!("Received: {:?}", msg);
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
//                let msg = WsResponse{ value: 333 };
//                let msg: Text = Json(&msg).into();
//                let msg = tungstenite::protocol::Message::Text(msg.unwrap());
//                println!("Sent message: {:?}", msg);
//                websocket.write_message(msg).unwrap();
            }
        });
    }
}

fn main() {
    thread::spawn(|| { start_web_server(); });
    start_websocket_server();
    //loop {};
}
