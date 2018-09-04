extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// Message via WebSocket
#[derive(Serialize, Deserialize, Debug)]
pub enum WsMessage {
    Data(Data),
    Command(Command),
    Ignore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub name: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Body {
    Vector(Vector),
    VectorPair(VectorPair),
    Matrix
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector {
    pub name_row: Option<Vec<String>>,
    pub data: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VectorPair {
    pub name_row_x: Option<Vec<String>>,
    pub name_row_y: Option<Vec<String>>,
    pub data_x: Vec<f32>,
    pub data_y: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Plot(PlotParam),
    ScatterPlot,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PlotParam {
    PlotParamForVector(PlotParamForVector),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlotParamForVector {
    pub data_name: String,
    pub area_name: String,
}

impl PlotParamForVector {
    pub fn into_wsmsg(self) -> WsMessage {
        WsMessage::Command(Command::Plot(PlotParam::PlotParamForVector(self)))
    }
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
