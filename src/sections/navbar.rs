use dioxus::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::{ScrollIntoViewOptions, window};

pub fn scroll_to_id(id: &str) {
    if let Some(document) = window().and_then(|w| w.document()) {
        if let Some(elem) = document.get_element_by_id(id) {
            let mut options = ScrollIntoViewOptions::new();
            options.behavior(web_sys::ScrollBehavior::Smooth);
            elem.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }
}

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! { 
        div { class:" overflow-y-hidden h-40 md:h-20 flex scroll-smooth bg-slate-900 text-slate-100 text-xl p-6 flex justify-right sticky top-0 z-50",
            div { class: "flex flex-row gap-x-3 md:gap-x-6 ml-auto items-center justify-center",
                h1 {
                class: "text-2xl font-semibold items-center justify-center absolute left-4",
                "Remy Sedlak"
            }   div { class:"hidden md:flex flex-row gap-x-3",

            
                button  {
                    onclick:  |_| scroll_to_id("hero"),
                    class: "hover:text-slate-300",
                    "Home"
                }
                button {
                    onclick:  |_| scroll_to_id("projects"),
                    class: "hover:text-slate-300",
                    "Projects"
                }
                button {
                    onclick:  |_| scroll_to_id("timeline"),
                    class: "hover:text-slate-300",
                    "Timeline"
                }
                button {
                    onclick:  |_| scroll_to_id("links"),
                    class: "hover:text-slate-300",
                    "Links"
                }
            }
            }
        }
    }
    }

