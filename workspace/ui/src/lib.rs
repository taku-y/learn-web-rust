extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate wdview_msg;

use std::collections::HashMap;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};
use yew::format::Text;
use wdview_msg::{WsMessage, DataFrame, Command, PlotParam, PlotParamArray};
pub mod msg;
use msg::{ModelMessage, WsMessageForModel};
use std::thread;
use stdweb::web::WebSocket;

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

        let mut model = Model {
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
        true
    }
}

fn process_wsmsg(model: &mut Model, wsmsg: WsMessage) {
    model.console.info("Received Websocket Message");
    model.console.info(&format!("{:?}", &wsmsg));

    match wsmsg {
        WsMessage::DataFrame(df) => {
            model.data.insert(df.name.clone(), df);
        }
        WsMessage::Command(command) => {
            model.commands.push(command);
            process_last_command(&model);
        }
        WsMessage::Connect(connect) => {
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

#[derive(Serialize)]
struct PlotlyDataArray(Vec<PlotlyData>);

#[derive(Serialize)]
struct PlotlyData {
    x: Vec<f64>,
    y: Vec<f64>,
}

js_serializable!( PlotlyData );
js_serializable!( PlotlyDataArray );

fn plot(model: &Model, params: &PlotParamArray) {
    let data = PlotlyDataArray(
        params.0.iter().map(|param| {
            let df = &model.data.get(&param.data_name).unwrap();
            PlotlyData {
                x: df.get_col(&param.col_name_x).unwrap().clone(),
                y: df.get_col(&param.col_name_y).unwrap().clone(),
            }
        }).collect()
    );

    // TODO: Move area_name to "layout" variable
    let area_name = &params.0[0].area_name;

    js! {
        Plotly.plot(
            document.getElementById(@{ &area_name }),
            @{ data },
            {
                margin: { t: 0 }
            }
        );
    }
}

fn process_last_command(model: &Model) {
    let command = model.commands.last().unwrap();

    use Command::{Plot};

    match command {
        Plot(plot_params) => { plot(model, plot_params); }
        _ => {}
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
