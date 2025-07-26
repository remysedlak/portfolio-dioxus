use crate::sections::{ Hero, Projects, Timeline, Links, Navbar, About};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! { 
        div { class:"h-[100dvh] overflow-y-auto flex flex-col ",
            Navbar {}
            div {class: "overflow-y-auto scroll-smooth ",
                Hero {}
                Projects {}
                Timeline {}
                About {}
                Links {}
            }
        }
    }
}
