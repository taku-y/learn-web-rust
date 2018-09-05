extern crate failure;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate wdview_msg;

use std::collections::HashMap;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};
use wdview_msg::{WsMessage, DataFrame, Command, PlotParam};
pub mod msg;
use msg::{ModelMessage, WsMessageForModel};

pub struct Model {
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    data: HashMap<String, DataFrame>,
    commands: Vec<Command>,
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
            commands: Vec::new(),
            ws: None,
            console: ConsoleService::new(),
        };
        console.info("Model was created");

        // Open websocket connection with callback
        let callback = model.link.send_back(
            |WsMessageForModel(msg)| { ModelMessage::WsMessage(msg) }
        );
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
        WsMessage::Ignore => {}
    }
}

fn plot(model: &Model, param: &PlotParam) {
    let df = &model.data.get(&param.data_name).unwrap();
    let xs = df.get_col(&param.col_name_x).unwrap();
    let ys = df.get_col(&param.col_name_y).unwrap();

    js! {
        var elem = document.getElementById( @{ &param.area_name });
        Plotly.plot(
            elem, [{
                x: @{ &xs },
                y: @{ &ys }
            }], {
                margin: { t: 0 }
            }
        );
    }
}

fn process_last_command(model: &Model) {
    let command = model.commands.last().unwrap();

    use Command::{Plot};

    match command {
        Plot(plot_param) => { plot(model, plot_param); }
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
            <script>
                { "document.getElementById('plot_area').innerHTML = 'aaaa!';" }
            </script>
            </div>
        }
    }
}

//impl Model {
//    fn view_data(&self) -> Html<Model> {
//        html! {
//            <p>{ "Data hasn't fetched yet." }</p>
//        }
////        if let Some(value) = self.data {
////            html! {
////                <p>{ value }</p>
////            }
////        } else {
////            html! {
////                <p>{ "Data hasn't fetched yet." }</p>
////            }
////        }
//    }
//
//
//}
