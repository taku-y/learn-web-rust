extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Data(Data),
    Command(Command),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
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
