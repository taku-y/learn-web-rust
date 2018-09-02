extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;
extern crate wdview_msg;

use yew::prelude::*;
use yew::format::Json;
use yew::services::{Task, ConsoleService};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};
pub mod msg;
use msg::{Msg, WsAction, WsRequest, MyData};

pub struct Model {
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    data: Option<u32>,
    ws: Option<WebSocketTask>,
    console: ConsoleService,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut console = ConsoleService::new();
        console.info("Model::create() was invoked");

        let mut model = Model {
            ws_service: WebSocketService::new(),
            link,
            data: Some(777 as u32),
            ws: None,
            console: console,
        };

        // Open websocket connection
        let callback = model.link.send_back(|data: MyData| {
            let mut console = ConsoleService::new();
            console.info("callback");
            console.info(&format!("{:?}", &data));
            Msg::Ignore
        });
//        let callback = model.link.send_back(move |Json(data)| {
//            let mut console = ConsoleService::new();
//            console.info("callback");
//            console.info(&format!("{:?}", &data));
//            Msg::WsReady(data)
//        });
        let notification = model.link.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                            }
                        });
        // TODO: Set websocket server address when accessing HTTP server
        let task = model.ws_service.connect("ws://0.0.0.0:9001/", callback, notification);
        model.ws = Some(task);

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.info("Model::update() was invoked");

        match msg {
            Msg::WsAction(action) => {
                match action {
                    WsAction::Connect => {
                        let callback = self.link.send_back(|Json(data)| Msg::WsReady(data));
                        let notification = self.link.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
                            }
                        });
                        let task = self.ws_service.connect("ws://0.0.0.0:9001/", callback, notification);
                        self.ws = Some(task);
                    }
                    WsAction::SendData(binary) => {
                        let request = WsRequest {
                            value: 321,
                        };
                        if binary {
                            self.ws.as_mut().unwrap().send_binary(Json(&request));
                        } else {
                            self.ws.as_mut().unwrap().send(Json(&request));
                        }
                    }
                    WsAction::Disconnect => {
                        self.ws.take().unwrap().cancel();
                    }
                    WsAction::Lost => {
                        self.ws = None;
                    }
                }
            }
            Msg::WsReady(response) => {
                self.console.info(&format!("{:?}", response));
                self.data = response.map(|data| data.value).ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                     { self.view_data() }
                    <button disabled=self.ws.is_some(),
                            onclick=|_| WsAction::Connect.into(),>{ "Connect To WebSocket" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::SendData(false).into(),>{ "Send To WebSocket" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::SendData(true).into(),>{ "Send To WebSocket [binary]" }</button>
                    <button disabled=self.ws.is_none(),
                            onclick=|_| WsAction::Disconnect.into(),>{ "Close WebSocket connection" }</button>
                </nav>
            </div>
        }
    }

}

impl Model {
    fn view_data(&self) -> Html<Model> {
        if let Some(value) = self.data {
            html! {
                <p>{ value }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }
}
