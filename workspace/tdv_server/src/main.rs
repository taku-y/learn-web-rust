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
extern crate url;
extern crate tungstenite;
extern crate yew;
extern crate tdv_ui;
extern crate tdv_msg;

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io;
use std::io::prelude::*;
use std::thread;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rocket::response::NamedFile;
use sled::Tree;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::{connect, WebSocket};
use tdv_msg::{WsMessage, DataFrame, Trace, PlotParam, Connect};
use url::Url;

// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/2
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

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
#[get("/tdv_ui.wasm")]
fn ugly_hack() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/tdv_ui.wasm")).ok()
}

fn start_web_server() {
    let path = String::from("data.db");
    let conf = sled::ConfigBuilder::new().path(path).build();
    let tree = Tree::start(conf).unwrap();
    let db_arc = Arc::new(tree);
    let routes = all_routes();
    rocket::ignite().mount("/", routes).manage(db_arc).launch();
}

fn who_are_you(websocket: &mut WebSocket<TcpStream>) -> WsMessage {
    let msg = WsMessage::WhoAreYou;
    let msg = tungstenite::protocol::Message::Text(serde_json::to_string(&msg).unwrap());
    websocket.write_message(msg).unwrap();

    let msg = websocket.read_message().unwrap();
    serde_json::from_str(msg.to_text().unwrap()).unwrap()
}

fn start_wdv_server() {
    let server = TcpListener::bind("0.0.0.0:9001").unwrap();
    let ws_ui = Arc::new(Mutex::new(Option::None));
    let ws_client = Arc::new(Mutex::new(Option::None));

    for stream in server.incoming() {
        let stream = stream.unwrap();
        println!("Made a connection: {:?}", &stream);

        let mut ws_ui_ = Arc::clone(&ws_ui);
        let mut ws_client_ = Arc::clone(&ws_client);

        spawn(move || {
            let mut websocket = accept(stream).unwrap();

            match who_are_you(&mut websocket) {
                WsMessage::IAmUI => {
                    {
                        let mut ws_ui = ws_ui_.lock().unwrap();
                        *ws_ui = Some(websocket);
                        println!("from UI");
                    }

                    // Just keep the websocket alive, used in the other thread
                    loop {}
                }
                WsMessage::IAmClient => {
                    {
                        let mut ws_client = ws_client_.lock().unwrap();
                        *ws_client = Some(websocket);
                        println!("from client");

                        // Unlock ws_client here
                    }

                    loop {
                        // Wait a request from the client
                        let mut ws_client = ws_client_.lock().unwrap();
                        let msg = (*ws_client).as_mut().unwrap().read_message().unwrap();
                        println!("Received: {:?} from client, send to UI", msg);

                        // Send a request to the UI to make a websocket from the UI to the client
                        let mut ws_ui = ws_ui_.lock().unwrap();
                        (*ws_ui).as_mut().unwrap().write_message(msg).unwrap();
                        println!("Sent");
                    }
                }
                _ => {}
            };
        });
    };
}

fn send_test_message<T>(websocket: &mut WebSocket<T>)
    where T: std::io::Read + std::io::Write {
    // Send a wdview message
    let msg1 = WsMessage::DataFrame(DataFrame {
        name: "3-dim vector".to_string(),
        columns: vec!["x".to_string(), "y".to_string()],
        index: vec![1, 2, 3, 4],
        data: vec![vec![5.0, 6.0, 7.0, 8.0],
                   vec![9.0, 12.0, 11.0, 10.0]],
    });
    let msg2 = PlotParam {
        area_name: "plot_area".to_string(),
        traces: vec![
            // 1st trace
            Trace {
                df_name: "3-dim vector".to_string(),
                col_name_x: "x".to_string(),
                col_name_y: "y".to_string()
            },
            // 2nd trace
            Trace {
                df_name: "3-dim vector".to_string(),
                col_name_x: "y".to_string(),
                col_name_y: "x".to_string()
            },
        ]
    }.into_command();

    let msg1 = tungstenite::protocol::Message::Text(serde_json::to_string(&msg1).unwrap());
    let msg2 = tungstenite::protocol::Message::Text(serde_json::to_string(&msg2).unwrap());
    println!("Following messages will be sent");
    println!("{:?}", &msg1);
    println!("{:?}", &msg2);
    websocket.write_message(msg1).unwrap();
    websocket.write_message(msg2).unwrap();
}

fn main() {
    thread::spawn(|| { start_web_server(); });
    thread::spawn(|| { start_wdv_server(); });

    // Client of wdview
    // Create TcpListener
    let client = TcpListener::bind("0.0.0.0:9002").unwrap();

    // Wait key press
    pause();

    // Request for wdview server to get the client to connect to the UI by WebSocket
    let (mut socket, response) = connect(Url::parse("ws://0.0.0.0:9001").unwrap())
        .expect("Can't connect");
    let msg = WsMessage::IAmClient;
    let msg = tungstenite::protocol::Message::Text(serde_json::to_string(&msg).unwrap());
    println!("{:?}", &msg);
    socket.write_message(msg).unwrap();

    // Send a request from the client to the UI to make a websocket
    let msg = WsMessage::Connect(Connect { address: "ws://0.0.0.0:9002".to_string() });
    let msg = tungstenite::protocol::Message::Text(serde_json::to_string(&msg).unwrap());
    socket.write_message(msg).unwrap();

    // Handshake with the UI
    for stream in client.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", &stream);
        let mut websocket = accept(stream).unwrap();

        send_test_message(&mut websocket);

        loop {}
    }
}