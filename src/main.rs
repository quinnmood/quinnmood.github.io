#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
mod navbar;
use navbar::Navbar;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-dvh w-full grow bg-gray-800 min-h-dvh",
            Navbar {

            },
            div {
                class: "w-full grow flex bg-gray-800",
            }


        }
    }
}
