extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// Message via WebSocket
#[derive(Serialize, Deserialize, Debug)]
pub enum WsMessage {
    DataFrame(DataFrame),
    Command(Command),
    Ignore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataFrame {
    // DataFrame meets conditions below:
    // data.len() == columns.len()
    // data[i].len() == index.len() for i = (0..columns.len())
    pub name: String,
    pub columns: Vec<String>,
    pub index: Vec<i64>,
    pub data: Vec<Vec<f64>>,
}

impl DataFrame {
    pub fn get_col(&self, col_name: &String) -> Option<&Vec<f64>> {
        for i in 0..self.columns.len() {
            if &self.columns[i] == col_name {
                return Some(&self.data[i]);
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Plot(PlotParam),
    ScatterPlot,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlotParam {
    pub data_name: String,
    pub area_name: String,
    pub col_name_x: String,
    pub col_name_y: String,
}

impl PlotParam {
    pub fn into_wsmsg(self) -> WsMessage {
        WsMessage::Command(Command::Plot(self))
    }
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
