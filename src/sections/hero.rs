use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div { class: "bg-slate-800 w-full h-[56dvh] justify-center items-center flex flex-col mb-auto p-4 text-white text-left md:text-center",
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            h1 { class: "text-4xl md:text-6xl font-semibold", "Hello, I am Remy Sedlak!" }
            h3 { class: "text-3xl mt-4", "Student and Researcher at the University of Pittsburgh" }
            
        }
    }
}
