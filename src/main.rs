#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            "Hi"
        }
    }
}
