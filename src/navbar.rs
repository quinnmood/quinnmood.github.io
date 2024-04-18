use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex shrink h-12 md:h-16 text-xl md:text-2xl  items-center px-2 relative shadow-lg bg-gray-900 rounded-b-lg",
            div {
                class: "relative flex z-10 flex-nowrap font-bold transition-all h-full items-center whitespace-nowrap *:px-2 hover:*:opacity-1/2 font-mono",
                div {
                    class: "relative h-full w-full shadow-lg",
                    button {
                        class: "peer h-full bg-gradient-to-r from-orange-300 to-red-400 text-transparent bg-clip-text",
                        "/ home ",
                    },
                    div {
                        class: "absolute top-0 z-[-1] h-0 peer-hover:h-40 hover:h-40 transition-all bg-gray-700 shadow-lg w-full skew-x-[-15deg] left-2 origin-top-left rounded-b-lg"
                    }

                },
                div {
                    class: "h-full relative w-full shadow-lg",
                    button {
                        class: "peer h-full bg-gradient-to-r from-orange-300 to-red-400 text-transparent bg-clip-text",
                        "/ quinnmood ",
                    },
                    div {
                        class: "absolute top-0 z-[-1] peer-[:hover]:h-40 peer-[:not(:hover):not(:focus)]:h-0 peer-[:hover]:opacity-1 origin-top-left peer-[:not(:hover)]:opacity-0 transition-all bg-gray-700 shadow-lg w-full skew-x-[-15deg] left-2 rounded-b-lg"
                    }

                },

                div {
                    class: "h-full relative shadow-lg",
                    button {
                        class: "peer h-full bg-gradient-to-r from-orange-300 to-red-400 text-transparent bg-clip-text",
                        "/ experience ",
                    },
                    div {
                        class: "absolute top-0 z-[-1] h-0 peer-hover:h-40 hover:h-40 transition-all bg-gray-700 shadow-lg w-full skew-x-[-15deg] left-2 origin-top-left rounded-b-lg"
                    }

                },
            }
        }
    }
}
