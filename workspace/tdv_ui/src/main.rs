extern crate yew;
extern crate stdweb;
extern crate ui;

use yew::prelude::*;
use stdweb::web::{INonElementParentNode, document};
use ui::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount(document().get_element_by_id("data_list").unwrap());
    yew::run_loop();
}
