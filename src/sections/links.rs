use dioxus::prelude::*;
use web_sys::window;


#[component]
pub fn Links() -> Element {
     let load_event_end = window()
        .expect("no global `window`")
        .performance()
        .expect("performance not available")
        .timing()
        .load_event_end();

    let navigation_start = window()
        .expect("no global `window`")
        .performance()
        .expect("performance not available")
        .timing()
        .navigation_start();

    let duration = if load_event_end > navigation_start {
        load_event_end - navigation_start
    } else {
        0.0
    };
    rsx! { div {
        class:"p-6 space-y-6 bg-yellow-50 relative",
        id:"links",
        h1 {
            class:"text-3xl  md:text-4xl font-bold mb-2 md:mb-4",
            "Links"
        }
        
        p {
            class:"absolute right-4 md:right-6 bottom-4 mb-auto font-light text-sm text-zinc-600 justify-right align-items-right",
            
            
            {"Assembled with "},
            a {
                href:"https://dioxuslabs.com/",
                target:"_blank",
                class:"hover:underline text-blue-500",
                {"Dioxus"}
            },
            img {
                class:"inline ml-1",
                src:asset!("/assets/icons/ferris.png"),
                style:"height: 1.3rem; width: 1.75rem;",
            },
            div {
            class: "text-sm block text-zinc-600 mt-2  flex flex-row",
            "Load time: " 
            p {  class:"font-mono ml-2 border rounded-md text-sm px-1 bg-zinc-300 hover:bg-zinc-200", 
                "{duration} ms"
            }
        }
            
            
        }
        ul { class:"text-xl",
            li {
                a {
                    href: "https://github.com/remysedlak",
                    target:"_blank",
                    class:"hover:underline",
                    {"GitHub"}
                }
                
                
            }
            li {
                a {
                    href: "https://www.linkedin.com/in/remysedlak/",
                    target:"_blank",
                    class:"hover:underline",
                    {"LinkedIn"}
                }
        
                
            }
            li {
                a {
                    target:"_blank",
                    class:"hover:underline",
                    href:"https://drive.google.com/file/d/1-gW3kevUAv6ImxSWIZgw-VO9A38-KFCk/view",
                    {"Resume"}
                }
                
            }
            li {
                a {
                    href:"mailto:remysedlak@gmail.com",
                    target:"_blank",
                    class:"hover:underline",
                    {"Email"}
                }
                
            }
        }
    }
}
}
