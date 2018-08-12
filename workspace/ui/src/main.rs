extern crate yew;
extern crate ui;

use yew::prelude::*;
use ui::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
