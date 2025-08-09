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
            class:"pt-6 px-6 bg-yellow-50 relative pb-2",
            id:"contact",
            h1 {
                class:"text-3xl  md:text-4xl font-bold mb-2 md:mb-3",
                "Contact"
            }

            ul { class:"text-xl  mb-2 md:mb-3",
                li {
                    a {
                        href: "https://github.com/remysedlak",
                        target:"_blank",
                        class:"hover:underline flex flex-row items-center inline-block",
                        p { class:"inline",
                            "GitHub"
                        }
                        img { class:"ml-1 inline",
                            src: asset!("/assets/icons/open-link.svg"),
                            style: "width:1rem;height:1rem;"
                        }
                    }
                }
                li {
                    a {
                        href: "https://www.linkedin.com/in/remysedlak/",
                        target:"_blank",
                        class:"hover:underline flex flex-row items-center inline-block transition-colors",
                        p { class:"inline",
                            "LinkedIn"
                        }
                        img { class:"ml-1 inline",
                            src: asset!("/assets/icons/open-link.svg"),
                            style: "width:1rem;height:1rem;"
                        }
                    }


                }
                li {
                    a {
                        target:"_blank",
                        class:"hover:underline flex flex-row items-center inline-block",
                        href:"https://drive.google.com/file/d/1-gW3kevUAv6ImxSWIZgw-VO9A38-KFCk/view",
                        p { class:"inline",
                            "Resume"
                        }
                        img { class:"ml-1 inline",
                            src: asset!("/assets/icons/open-link.svg"),
                            style: "width:1rem;height:1rem;"
                        }
                    }

                }
                li {
                    a {
                        href:"mailto:remysedlak@gmail.com",
                        target:"_blank",
                        class:"hover:underline flex flex-row items-center inline-block",
                        p { class:"inline",
                            "Email"
                        }
                        img { class:"ml-1 inline",
                            src: asset!("/assets/icons/open-link.svg"),
                            style: "width:1rem;height:1rem;"
                        }
                    }

                }
            }

        }
        div {class:"text-zinc-600 gap-x-1 md:gap-x-2 font-light flex bg-yellow-50 py-1 text-center items-center justify-center text-xs md:text-sm",


        p {
            "Remy Sedlak © 2025"
        },
        p {
            "•"
        },
        div { class:"flex flex-row gap-x-1 items-center justify-center",
                
        p {
                {"Assembled with "},
        }

                a {
                    href:"https://dioxuslabs.com/",
                    target:"_blank",
                    class:"hover:underline text-blue-500 inline",
                    {"Dioxus"}
                },
                p { class:"hidden md:inline",
                    " in "
                },

                p {  class:"hidden md:inline text-center",
                    "{duration} ms"
                }
                img {
                    class:"md:hidden inline ml-1 w-5 h-4",
                    src:asset!("/assets/icons/ferris.png"),
                },
                img {
                    class:"hidden md:inline md:w-7 md:h-5 mt-1 ml-[1/2]",
                    src:asset!("/assets/icons/ferris.png"),
                },
            }
        }
    }
}
