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
use wdview_msg::{WsMessage, Data, Command, PlotParam, PlotParamForVector, Body};
pub mod msg;
use msg::{ModelMessage, WsMessageForModel};

pub struct Model {
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    data: HashMap<String, Data>,
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
        WsMessage::Data(data) => {
            model.data.insert(data.name.clone(), data);
        }
        WsMessage::Command(command) => {
            model.commands.push(command);
            process_last_command(&model);
        }
        WsMessage::Ignore => {}
    }
}

fn plot_for_vector(model: &Model, param: &PlotParamForVector) {
    let data_body = &model.data.get(&param.data_name).unwrap().body;

    match data_body {
        Body::Vector(v) => {
            let x: Vec<f32> = (0..v.data.len()).map(|x| x as f32).collect();
            let y = &v.data;
            let area_name = &param.area_name;

            js! {
                var elem = document.getElementById( @{ area_name });
                Plotly.plot(
                    elem, [{
                        x: @{ x }, //[1, 2, 3, 4, 5],
                        y: @{ y }
                    }], { //[1, 2, 4, 8, 16] }], {
                        margin: { t: 0 }
                    }
                );
            }
        }
        _ => {} // TODO: make alert for type mismatch
    }
}

fn process_last_command(model: &Model) {
    let command = model.commands.last().unwrap();

    use Command::*;
    use PlotParam::*;

    match command {
        Plot(plot_param) => {
            match plot_param {
                PlotParamForVector(param) => { plot_for_vector(model, param); }
            }
        }
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
