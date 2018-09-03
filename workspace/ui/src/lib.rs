extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate yew;
extern crate wdview_msg;

use std::collections::HashMap;
use yew::prelude::*;
use yew::format::{Json, Text, Binary};
use yew::services::{Task, ConsoleService};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};
use wdview_msg::{WsMessage, Data};
pub mod msg;
use msg::{ModelMessage};

pub struct Model {
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    data: HashMap<String, Data>,
    ws: Option<WebSocketTask>,
    console: ConsoleService,
}

struct WsMessageForModel(WsMessage);

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

impl Component for Model {
    type Message = ModelMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut console = ConsoleService::new();
        console.info("Model::create() was invoked");

        let mut model = Model {
            ws_service: WebSocketService::new(),
            link,
            data: HashMap::new(),
            ws: None,
            console: ConsoleService::new(),
        };
        console.info("Model was created");

        // Open websocket connection
        let callback = model.link.send_back(
            |WsMessageForModel(msg)| {ModelMessage::WsMessage(msg)
        });
        let notification = model.link.send_back(|status| {
            match status {
                WebSocketStatus::Opened => ModelMessage::Ignore,
                WebSocketStatus::Closed | WebSocketStatus::Error => ModelMessage::Ignore,
//                WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),

            }
        });
        console.info("Closures were created");
        // TODO: Set websocket server address when accessing HTTP server
        let task = model.ws_service.connect("ws://0.0.0.0:9001/", callback, notification);
        model.ws = Some(task);
        console.info("Handshake");

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.info("Model::update() was invoked");

        match msg {
            ModelMessage::WsMessage(wsmsg) => {
                match wsmsg {
                    WsMessage::Data(data) => {
                        self.console.info(&format!("{:?}", &data));
                    }
                    WsMessage::Command(command) => {
                        self.console.info(&format!("{:?}", &command));
                    }
                    WsMessage::Ignore => {
                        self.console.info(&format!("Ignore"));
                    }
//                    WsAction::Connect => {
//                        let callback = self.link.send_back(|Json(data)| Msg::WsReady(data));
//                        let notification = self.link.send_back(|status| {
//                            match status {
//                                WebSocketStatus::Opened => Msg::Ignore,
//                                WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
//                            }
//                        });
//                        let task = self.ws_service.connect("ws://0.0.0.0:9001/", callback, notification);
//                        self.ws = Some(task);
//                    }
//                    WsAction::SendData(binary) => {
//                        let request = WsRequest {
//                            value: 321,
//                        };
//                        if binary {
//                            self.ws.as_mut().unwrap().send_binary(Json(&request));
//                        } else {
//                            self.ws.as_mut().unwrap().send(Json(&request));
//                        }
//                    }
//                    WsAction::Disconnect => {
//                        self.ws.take().unwrap().cancel();
//                    }
//                    WsAction::Lost => {
//                        self.ws = None;
//                    }
//                }
                }
            }
            ModelMessage::UiMessage(_) => {
                self.console.info(&format!("UiMessage"));
            }
            ModelMessage::Ignore => {
                self.console.info(&format!("Ignore"));
            }
        }
//            Msg::WsReady(response) => {
//                self.console.info(&format!("{:?}", response));
//                self.data = response.map(|data| data.value).ok();
//            }
//            Msg::Ignore => {
//                return false;
//            }

        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
//                <nav class="menu",>
//                     { self.view_data() }
//                    <button disabled=self.ws.is_some(),
//                            onclick=|_| WsAction::Connect.into(),>{ "Connect To WebSocket" }</button>
//                    <button disabled=self.ws.is_none(),
//                            onclick=|_| WsAction::SendData(false).into(),>{ "Send To WebSocket" }</button>
//                    <button disabled=self.ws.is_none(),
//                            onclick=|_| WsAction::SendData(true).into(),>{ "Send To WebSocket [binary]" }</button>
//                    <button disabled=self.ws.is_none(),
//                            onclick=|_| WsAction::Disconnect.into(),>{ "Close WebSocket connection" }</button>
//                </nav>
            </div>
        }
    }

}

impl Model {
    fn view_data(&self) -> Html<Model> {
        html! {
            <p>{ "Data hasn't fetched yet." }</p>
        }
//        if let Some(value) = self.data {
//            html! {
//                <p>{ value }</p>
//            }
//        } else {
//            html! {
//                <p>{ "Data hasn't fetched yet." }</p>
//            }
//        }
    }


}
