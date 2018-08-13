extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use failure::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::{Task, ConsoleService};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};

type AsBinary = bool;

pub enum WsAction {
    Connect,
    SendData(AsBinary),
    Disconnect,
    Lost,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Serialize, Debug)]
pub struct WsResponse {
    pub value: u32,
}

pub enum Msg {
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
}

pub struct Model {
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    data: Option<u32>,
    ws: Option<WebSocketTask>,
    console: ConsoleService,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: create connection to the websocket server here
        let mut console = ConsoleService::new();
        console.info("Model::create() was invoked");

        Model {
            ws_service: WebSocketService::new(),
            link,
            data: Some(777 as u32),
            ws: None,
            console: console,
        }
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
