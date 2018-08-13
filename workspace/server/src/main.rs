#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unused_variables)]
#![allow(unused_mut)]
//#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![feature(proc_macro_non_items)]
#![feature(use_extern_macros)]

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

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::Serialize;
use std::thread;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
//use bincode::{deserialize, serialize};
#[macro_use]
use maud::{html, Markup};
//use rocket::response::status;
use rocket::response::NamedFile;
use rocket::State;
//use rocket_contrib::Json;
use sled::{ConfigBuilder, Tree};
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use yew::format::{Text, Json};
use ui::WsResponse;

fn all_routes() -> Vec<rocket::Route> {
    routes![
        index,
        static_file,
        ugly_hack,
//        create_task,
//        get_task,
//        get_tasks,
//        update_all_tasks
    ]
}

/// This is the entrypoint for our yew client side app.
#[get("/")]
fn index(db: State<Arc<sled::Tree>>) -> Markup {
    // maud macro
    html! {
        // link rel="stylesheet" href="static/styles.css" {}
        body {}
        // yew-generated javascript attaches to <body>
        script src=("static/ui.js") {}
    }
}

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

fn main() {
    thread::spawn(|| { start_web_server(); });
    start_websocket_server();
    //loop {};
}
