#![recursion_limit="128"]

extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate tdv_msg;

pub mod msg;
pub mod plot;

use std::collections::HashMap;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketTask};
use tdv_msg::{WsMessage, DataFrame, Command};
use msg::{ModelMessage, WsMessageForModel};
use plot::plot;

pub struct Model {
    link: ComponentLink<Model>,
    data: HashMap<String, DataFrame>,
    commands: Vec<Command>,
    ws_server: WebSocketTask,
    ws_client: Option<WebSocketTask>, // client will come later thus be Option
    console: ConsoleService,
}

impl Component for Model {
    type Message = ModelMessage;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut console = ConsoleService::new();
        console.info("Model::create() was invoked");

        // Open websocket for accepting connection request from client of the viewer
        let server_address = "ws://0.0.0.0:9001";
        let callback = link.send_back(
            |WsMessageForModel(msg)| { ModelMessage::WsMessage(msg) }
        );
        let notification = link.send_back(|_status| { ModelMessage::Ignore });
        let ws_server = WebSocketService::new().connect(server_address, callback.into(),
                                                        notification);
        console.info("ws_thread was created");

        let model = Model {
            link: link,
            data: HashMap::new(),
            commands: Vec::new(),
            ws_server,
            ws_client: None,
            console: ConsoleService::new(),
        };
        console.info("Model was created");
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.console.info("Model::update() was invoked");

        match msg {
            ModelMessage::WsMessage(wsmsg) => { process_wsmsg(self, wsmsg); }
            ModelMessage::UiMessage(_) => { self.console.info(&format!("UiMessage")); }
            ModelMessage::Ignore => { self.console.info(&format!("Ignore")); }
        }

        // Always update the view, but it can be inefficient
        // TODO: Fix here if in some cases the view shouldn't be updated
        true
    }
}

fn process_wsmsg(model: &mut Model, wsmsg: WsMessage) {
    model.console.info("Received Websocket Message");
    model.console.info(&format!("{:?}", &wsmsg));

    use WsMessage::*;

    match wsmsg {
        DataFrame(df) => {
            model.data.insert(df.name.clone(), df);
        }

        Command(command) => {
            model.commands.push(command);
            process_last_command(model);
        }

        Connect(connect) => {
            let client_address = connect.address;
            let callback = model.link.send_back(
                |WsMessageForModel(msg)| { ModelMessage::WsMessage(msg) }
            );
            let notification = model.link.send_back(|_status| { ModelMessage::Ignore });
            model.ws_client = Some(WebSocketService::new().connect(&client_address,
                                                                   callback.into(), notification));
            model.console.info("Finished");
        }
        WsMessage::WhoAreYou => { model.ws_server.send(WsMessage::IAmUI); }
        WsMessage::Ignore => {}
        WsMessage::IAmUI => {}
        WsMessage::IAmClient => {}
    }
}

fn process_last_command(model: &Model) {
    let command = model.commands.last().unwrap();

    use Command::{Plot};

    match command {
        Plot(plot_params) => { plot(&model.data, plot_params); }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
            <table style="overflow-y=scroll",>
            {
                for self.data.iter().map(|item| html! {
                    <tr><td> { item.0 } </td></tr>
                })
            }
            </table>
            </div>
        }
    }
}
