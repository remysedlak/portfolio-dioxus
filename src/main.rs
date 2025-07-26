use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod pages;
mod sections;
use pages::{Home};
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            
            Home {},
        }
    }
}
