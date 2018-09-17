extern crate yew;
extern crate stdweb;
extern crate tdv_ui;

use yew::prelude::*;
use stdweb::web::{INonElementParentNode, document};
use tdv_ui::Model;

fn main() {
    yew::initialize();
    // TODO: check if tdv_data_list is the right tag to mount
    App::<Model>::new().mount(document().get_element_by_id("tdv_data_list").unwrap());
    yew::run_loop();
}
