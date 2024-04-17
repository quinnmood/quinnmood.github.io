#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
const _CSS_URL: &str = manganis::mg!(file("assets/main.css"));

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            class: "flex w-full h-full bg-green-400 text-lg",
            "HIIII"
        }
    }
}
