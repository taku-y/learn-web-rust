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
use msg::{ModelMessage, WsMessageForModel};

pub struct Model {
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    data: HashMap<String, Data>,
    ws: Option<WebSocketTask>,
    console: ConsoleService,
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
                        self.data.insert(data.name.clone(), data);
                    }
                    WsMessage::Command(command) => {
                        self.console.info(&format!("{:?}", &command));
                    }
                    WsMessage::Ignore => {
                        self.console.info(&format!("Ignore"));
                    }
                }
            }
            ModelMessage::UiMessage(_) => {
                self.console.info(&format!("UiMessage"));
            }
            ModelMessage::Ignore => {
                self.console.info(&format!("Ignore"));
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <table style="overflow-y=scroll",>
            {
                for self.data.iter().map(|item| html! {
                    <tr><td> { item.0 } </td></tr>
                })
            }
            </table>
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
