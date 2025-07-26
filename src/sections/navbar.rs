use dioxus::prelude::*;
use web_sys::{ScrollIntoViewOptions, window};

pub fn scroll_to_id(id: &str) {
    if let Some(document) = window().and_then(|w| w.document()) {
        if let Some(elem) = document.get_element_by_id(id) {
            let options = ScrollIntoViewOptions::new();
            options.set_behavior(web_sys::ScrollBehavior::Smooth);
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
    let mut menu_open = use_signal(|| false);

    let toggle_menu = move |_| {
        menu_open.set(!menu_open());
    };

    let close_menu_and_scroll = move |id: &str| {
        let id = id.to_string();
        move |_| {
            menu_open.set(false);
            scroll_to_id(&id);
        }
    };

    rsx! {
        div { 
            class: "overflow-y-hidden h-40 md:h-20 flex scroll-smooth bg-slate-900 text-slate-100 text-xl p-6  flex justify-right sticky top-0 z-50",
            div { 
                class: "flex flex-row gap-x-3 md:gap-x-10 ml-auto items-center justify-center",
                h1 {
                    class: "text-3xl font-semibold items-center justify-center absolute left-4",
                    "Remy Sedlak"
                }
                
                // Mobile menu button
                button {
                    class: "md:hidden h-8 inline absolute right-4 hover:opacity-75",
                    onclick: toggle_menu,
                    img {
                        src: asset!("/assets/icons/menu.svg"),
                        class: "h-8"
                    }
                }
                
                // Desktop menu (hidden on mobile)
                div { 
                    class: "hidden md:flex flex-row gap-x-5 text-xl",
                    button {
                        onclick: |_| scroll_to_id("hero"),
                        class: "hover:cursor-pointer hover:text-slate-400  transition-colors",
                        "Home"
                    }
                    button {
                        onclick: |_| scroll_to_id("projects"),
                        class: "hover:cursor-pointer hover:text-slate-400  transition-colors",
                        "Projects"
                    }
                    button {
                        onclick: |_| scroll_to_id("timeline"),
                        class: "hover:cursor-pointer hover:text-slate-400  transition-colors",
                        "Timeline"
                    }
                    button {
                        onclick: |_| scroll_to_id("about"),
                        class: "hover:cursor-pointer hover:text-slate-400  transition-colors",
                        "About",
                    },           
                }
            }
        }
        
        // Mobile dropdown menu
        if menu_open() {
            div {
                class: "md:hidden  bg-slate-900 text-slate-100 absolute top-12 left-0 right-0 z-40 border-t border-slate-700",
                
                div {
                    
                    class: "flex flex-col p-2 gap-y-4 ",
                    hr {
                        class:"text-slate-500"
                    }
                    button {
                        onclick: close_menu_and_scroll("hero"),
                        class: "font-semibold border text-center text-lg py-2 px-2 hover:bg-slate-800 rounded transition-colors",
                        "Home"
                    }
                    button {
                        onclick: close_menu_and_scroll("projects"),
                        class: "font-semibold border text-center text-lg py-2 px-2 hover:bg-slate-800 rounded transition-colors",
                        "Projects"
                    }
                    button {
                        onclick: close_menu_and_scroll("timeline"),
                        class: "font-semibold border text-center text-lg py-2 px-2 hover:bg-slate-800 rounded transition-colors",
                        "Timeline"
                    }
                    button {
                        onclick: close_menu_and_scroll("about"),
                        class: "font-semibold border text-center text-lg py-2 px-2 hover:bg-slate-800 rounded transition-colors",
                        "About Me"
                    }
                }
            }
        }
    }
}