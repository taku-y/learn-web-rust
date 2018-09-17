//! # tdv_msg
//!
//! `tdv_msg` is a collection structs to communicate with the server and
//! the user interface (web browser) of Tiny Data Viewer.

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate failure;

pub mod grid;
pub use grid::*;

/// Message for WebSocket.
#[derive(Serialize, Deserialize, Debug)]
pub enum WsMessage {
    /// Send data frame to the UI.
    DataFrame(DataFrame),
    /// Send command to the UI.
    Command(Command),
    /// Request to establish websocket from the UI to the client.
    Connect(Connect),
    /// Ask you are the UI or the client.
    WhoAreYou,
    /// Reply from the UI.
    IAmUI,
    /// Reply from the client.
    IAmClient,
    /// Ignore this message.
    Ignore,
}

impl From<WsMessage> for Result<String, failure::Error> {
    fn from(msg: WsMessage) -> Result<String, failure::Error> {
        Ok(serde_json::to_string(&msg).unwrap())
    }
}

/// Data frame like `pandas` in Python.
///
/// DataFrame should satisfy the conditions below:
/// ```
/// data.len() == columns.len()
/// data[i].len() == index.len() for i = (0..columns.len())
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct DataFrame {
    /// Name of this data frame.
    pub name: String,
    /// Set of names of columns.
    pub columns: Vec<String>,
    /// Indices of rows.
    pub index: Vec<i64>,
    /// The body of the data frame.
    pub data: Vec<Vec<f64>>,
}

impl DataFrame {
    /// Get the column vector specified by the `col_name`.
    pub fn get_col(&self, col_name: &String) -> Option<&Vec<f64>> {
        for i in 0..self.columns.len() {
            if &self.columns[i] == col_name {
                return Some(&self.data[i]);
            }
        }
        None
    }
}

/// Represent commands for the UI.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Plot data using Plotly.
    Plot(PlotParam),
    /// Set grid layout.
    SetGridLayout(GridLayout),
}

/// Represent request to make websocket between the UI and the client.
#[derive(Serialize, Deserialize, Debug)]
pub struct Connect {
    /// Address of the client, e.g., `0.0.0.0:9002`.
    pub address: String,
}

/// Represent plotting parameters.
#[derive(Serialize, Deserialize, Debug)]
pub struct PlotParam {
    /// HTML element on which the plot is created.
    pub area_name: String,
    /// Collection of traces, which is following the convention of data in
    /// Plotly.
    pub traces: Vec<Trace>,
}

impl PlotParam {
    /// Create `WsMessage::Command` object from `Command::Plot`. It is just a
    /// utility function.
    pub fn into_command(self) -> WsMessage {
        WsMessage::Command(Command::Plot(self))
    }
}

impl GridLayout {
    /// Create `WsMessage::Command` object from `Command::Plot`. It is just a
    /// utility function.
    pub fn into_command(self) -> WsMessage {
        WsMessage::Command(Command::SetGridLayout(self))
    }
}

/// A trace, which is following the convention of data in Plotly.
#[derive(Serialize, Deserialize, Debug)]
pub enum Trace {
    /// A scatter plot.
    Scatter(Scatter),
}

/// Parameters of a scatter plot.
#[derive(Serialize, Deserialize, Debug)]
pub struct Scatter {
    /// The name of data frame, which is already sent to the UI before making
    /// a scatter plot.
    pub df_name: String,
    /// The name of column as x-axis of the scatter plot.
    pub col_name_x: String,
    /// The name of column as y-axis of the scatter plot.
    pub col_name_y: String,
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
