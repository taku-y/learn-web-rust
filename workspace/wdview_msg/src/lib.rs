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
    pub data: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Plot2D,
    ScatterPlot,
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
