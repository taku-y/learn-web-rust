extern crate tdv_client;
extern crate tdv_msg;

use tdv_client::TdvClient;
use tdv_msg::{WsMessage, DataFrame, PlotParam, Trace, Scatter, GridLayout,
              Row, Col};

fn main() {
    let mut client = TdvClient::new("0.0.0.0:9001".to_string(),
                                    "0.0.0.0:9002".to_string());

    client.connect_to_ui();

    let msg_df = WsMessage::DataFrame(DataFrame {
        name: "3-dim vector".to_string(),
        columns: vec!["x".to_string(), "y".to_string()],
        index: vec![1, 2, 3, 4],
        data: vec![vec![5.0, 6.0, 7.0, 8.0],
                   vec![9.0, 12.0, 11.0, 10.0]],
    });
    let msg_gl = GridLayout::new(3)
        .add("grid_elem1".to_string(), Row(1), Col(1))
        .add("grid_elem2".to_string(), Row(1), Col(2))
        .add("grid_elem3".to_string(), Row(2), Col((1, 2)))
        .add("grid_elem4".to_string(), Row((1, 2)), Col(3))
        .into_command();
    let msg_plot = |id: String| {
        PlotParam {
            area_name: id,
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
        }.into_command()
    };

    client.send_ws_message(msg_df);
    client.send_ws_message(msg_gl);
    client.send_ws_message(msg_plot("grid_elem1".to_string()));
    client.send_ws_message(msg_plot("grid_elem2".to_string()));
    client.send_ws_message(msg_plot("grid_elem3".to_string()));
    client.send_ws_message(msg_plot("grid_elem4".to_string()));
}