use dioxus::prelude::*;

#[component]
pub fn Links() -> Element {
    rsx! { div {
        class:"p-6 space-y-6 bg-yellow-50 relative",
        id:"links",
        h1 {
            class:"text-3xl md:text-4xl font-bold mb-2 md:mb-5",
            "Links"
        }
        p {
            class:"absolute right-4 bottom-4 mb-auto font-light text-sm",
            {"Assembled with "},
            a {
                href:"https://dioxuslabs.com/",
                target:"_blank",
                class:"hover:underline",
                {"Dioxus"}
            },
            img {
                class:"inline ml-1 h-4",
                src:asset!("/assets/icons/ferris.png")
            },
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
