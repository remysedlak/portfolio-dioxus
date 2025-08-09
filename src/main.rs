use dioxus::prelude::*;
use dioxus::document::{Link, Stylesheet};
use pages::Home;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod pages;
mod sections;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Stylesheet { href: TAILWIND_CSS }
        Link { rel: "icon", href: FAVICON }
        Home {}
    }
}
