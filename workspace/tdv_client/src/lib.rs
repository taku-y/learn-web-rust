extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate tungstenite;
extern crate url;
extern crate tdv_msg;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use tungstenite::server::accept;
use tungstenite::client::AutoStream;
use tungstenite::{connect, WebSocket};
use url::Url;
use tdv_msg::{WsMessage, Connect};

fn send_ws_message<T>(socket: &mut WebSocket<T>, msg: WsMessage)
    where T: Read + Write {
    let msg = tungstenite::protocol::Message::Text(
        serde_json::to_string(&msg).unwrap()
    );
    println!("{:?}", &msg);
    socket.write_message(msg).unwrap();
}

pub struct TdvClient {
    address_client: String,
    listener: TcpListener,
    socket_server: WebSocket<AutoStream>,
    socket_ui: Option<WebSocket<TcpStream>>,
}

impl TdvClient {
    pub fn new(address_server: String, address_client: String) -> Self {
        // Client's TCP listener
        let listener = TcpListener::bind(address_client.as_str()).unwrap();

        // Connect to TDV server
        let uri_server = format!("ws://{}", address_server);
        let (mut socket_server, _response) =
            connect(Url::parse(&uri_server).unwrap())
                .expect("Can't connect");
        let msg = WsMessage::IAmClient;
        send_ws_message(&mut socket_server, msg);

        TdvClient {
            address_client,
            listener,
            socket_server,
            socket_ui: None
        }
    }

    pub fn connect_to_ui(&mut self) {
        // Send a request from the client to the UI to make a websocket
        let uri = format!("ws://{}", &self.address_client);
        let msg = WsMessage::Connect(Connect { address: uri.to_string() });
        send_ws_message(&mut self.socket_server, msg);

        // Websocket handshake
        for stream in self.listener.incoming() {
            // TODO: Implement error handling
            let stream = stream.unwrap();
            println!("{:?}", &stream);
            let websocket = accept(stream).unwrap();
            self.socket_ui = Some(websocket);
            break;
        }
    }

    pub fn send_ws_message(&mut self, msg: WsMessage) {
        send_ws_message(self.socket_ui.as_mut().unwrap(), msg);
    }
}
