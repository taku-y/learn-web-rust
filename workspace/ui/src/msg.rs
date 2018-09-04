// This module defines messages for yew Model.

extern crate failure;
extern crate serde_json;
extern crate wdview_msg;

use failure::Error;
use yew::format::{Text, Binary};
use wdview_msg::{WsMessage, Data, Command};

pub enum UiMessage {
    Ignore,
}

pub enum ModelMessage {
    WsMessage(WsMessage),
    UiMessage(UiMessage),
    Ignore,
}

pub struct WsMessageForModel(pub WsMessage);

impl From<Text> for WsMessageForModel {
    fn from(text: Text) -> WsMessageForModel {
        WsMessageForModel(
            serde_json::from_str(&text.unwrap()).unwrap()
        )
    }
}

impl From<Binary> for WsMessageForModel {
    fn from(bin: Binary) -> WsMessageForModel {
        WsMessageForModel(WsMessage::Ignore)
    }
}
