extern crate tdv_client;
extern crate tdv_msg;

use tdv_client::TdvClient;
use tdv_msg::{WsMessage, DataFrame, PlotParam, Trace, Scatter, Command};

fn main() {
    let mut client = TdvClient::new("0.0.0.0:9001".to_string(),
                                    "0.0.0.0:9002".to_string());

    client.connect_to_ui();

    let msg1 = WsMessage::DataFrame(DataFrame {
        name: "3-dim vector".to_string(),
        columns: vec!["x".to_string(), "y".to_string()],
        index: vec![1, 2, 3, 4],
        data: vec![vec![5.0, 6.0, 7.0, 8.0],
                   vec![9.0, 12.0, 11.0, 10.0]],
    });
    let msg2 = PlotParam {
        area_name: "plot_area".to_string(),
        traces: vec![
            // 1st trace
            Trace::Scatter(Scatter {
                df_name: "3-dim vector".to_string(),
                col_name_x: "x".to_string(),
                col_name_y: "y".to_string()
            }),
            // 2nd trace
            Trace::Scatter(Scatter {
                df_name: "3-dim vector".to_string(),
                col_name_x: "y".to_string(),
                col_name_y: "x".to_string()
            }),
        ]
    }.into_command();

    client.send_ws_message(msg1);
    client.send_ws_message(msg2);

    let msg3 = WsMessage::Command(Command::UpdateStyle("hey!".to_string()));
    client.send_ws_message(msg3);
}
