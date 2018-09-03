// This module defines messages for yew Model.

extern crate failure;
//#[macro_use]
//extern crate serde_derive;
//#[macro_use]
//extern crate yew;
extern crate wdview_msg;

use failure::Error;
use yew::format::{Text, Binary};
use wdview_msg::{WsMessage, Data, Command};

type AsBinary = bool;

//pub enum WsAction {
//    Connect,
//    SendData(AsBinary),
//    Disconnect,
//    Lost,
//}

//impl From<WsAction> for Msg {
//    fn from(action: WsAction) -> Self {
//        Msg::WsAction(action)
//    }
//}

///// This type is used as a request which sent to websocket connection.
//#[derive(Serialize, Debug)]
//pub struct WsRequest {
//    pub value: u32,
//}
//
///// This type is an expected response from a websocket connection.
//#[derive(Deserialize, Serialize, Debug)]
//pub struct WsResponse {
//    pub value: u32,
//}

//#[derive(Debug)]
//pub enum MyData {
//    String(String),
//    Binary(Vec<u8>)
//}
//
//impl From<Text> for MyData {
//    fn from(text: Text) -> Self {
//        MyData::String(text.unwrap())
//    }
//}
//
//impl From<Binary> for MyData {
//    fn from(bin: Binary) -> Self {
//        MyData::Binary(bin.unwrap())
//    }
//}

pub enum UiMessage {
    Ignore,
}

pub enum ModelMessage {
    WsMessage(WsMessage),
    UiMessage(UiMessage),
    Ignore,
}
