use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StackItem {
    pub name: &'static str,
    pub color: &'static str,
    pub link: &'static str,
    pub img_url: Asset,
}

#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub stack: Vec<StackItem>,
    pub repository: &'static str,
    pub desc: &'static str,
    pub img_url: Asset,
}

#[component]
pub fn Projects() -> Element {
    let portfolio = get_portfolio();

    rsx! {
        div { class:"bg-slate-200 w-full p-4 md:p-8 pt-8 ",
        id:"projects",
            h1 { class: "text-4xl font-bold mb-4 md:mb-6", "My Projects" }
            // Projects grid
            div { class: "mx-auto",
            
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    for project in portfolio {
                        ProjectCard { project: project }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(project: Project) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow duration-300",
            // Project image
            div { class: "relative overflow-hidden bg-blue-700",
                img {
                    style: "height: 12rem; width: 28rem;",
                    src: "{project.img_url}",
                    loading:"lazy",
                    alt: "{project.title}",
                    class: "border-b border-slate-300 w-full h-full object-cover hover:scale-105 transition-transform duration-300"
                }
            }

            // Project content
            div { class: "p-4",
                // Title
                div { class:"flex flex-row items-center",
                h3 { class: "text-xl font-bold text-gray-800 mb-2", "{project.title}" }
                for stack_item in project.stack {
                            a {
                                href: "{stack_item.link}",
                                target: "_blank",
                                title:"{stack_item.name}",
                                class:"group",
                            img {
                                src: "{stack_item.img_url}",
                                style:"width: 1.5rem; height: 1.5rem;",
                                class: "ml-3 mb-1 gap-x-2 transition-transform duration-200 group-hover:scale-110 group-hover:drop-shadow"
                            }
                        }
                    }
                }
                hr {
                    class:"text-slate-300 pt-2"
                }
                // Description (with HTML support)
                div {
                    class: "text-gray-600 mb-4 md:text-sm leading-relaxed",
                    dangerous_inner_html: "{project.desc}"
                }

                // Repository link
                div { class: "flex justify-end",
                    a {
                        href: "{project.repository}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "inline-flex  items-center text-blue-500 text-sm font-medium rounded-md hover:underline  transition-colors duration-200",
                        // GitHub icon (using Unicode)
                        "View on GitHub"
                    }
                }
            }
        }
    }
}

pub fn get_portfolio() -> Vec<Project> {
    vec![
        Project {
            title: "The Inqusitor",
            img_url: asset!("/assets/snapshots/inquisitor.png"),
            stack: vec![
                StackItem { img_url: asset!("/assets/icons/stack/python.svg"), name: "python", color: "#B3D3F2", link: "https://www.python.org/" },
                StackItem { img_url: asset!("/assets/icons/stack/js.svg"),name: "js", color: "#FFD699", link: "https://developer.mozilla.org/en-US/docs/Web/JavaScript" },
                StackItem { img_url: asset!("/assets/icons/stack/html5.svg"),name: "html5", color: "#FFD699", link: "https://developer.mozilla.org/en-US/docs/Web/HTML" },
            ],
            repository: "https://github.com/DSmith215/The-Inquistor",
            desc: "The Inquisitor is a demo extension that detects <b>deepfake images</b> of politicians to stop <b>misinformation</b> online. Users can optimize filters on the module.",
        },
        Project {
            title: "Reflexa",
            img_url: asset!("/assets/snapshots/reflexa.png"),
            stack: vec![
                StackItem { img_url: asset!("/assets/icons/stack/react.svg"),name: "react", color: "#B3ECFF", link: "https://react.dev/" },
                StackItem { img_url: asset!("/assets/icons/stack/django.svg"),name: "django", color: "#B3C6B3", link: "https://www.djangoproject.com/" },
                StackItem { img_url: asset!("/assets/icons/stack/postgresql.svg"),name: "postgresql", color: "#B3C6E0", link: "https://www.postgresql.org/" },
                StackItem { img_url: asset!("/assets/icons/stack/ec2.svg"),name: "ec2", color: "#FFE5B4", link: "https://aws.amazon.com/ec2/" },
            ],
            repository: "https://github.com/remysedlak/Reflexa",
            desc: "Reflexa is a <b>full stack web application</b> designed to analyze <b>mental health</b> journal entries over time. Data analysis is used to evaluate journal content and emotion.",
        },
        Project {
            title: "TkinterAudio",
            img_url: asset!("/assets/snapshots/tkinter.png"),
            stack: vec![
                StackItem { img_url: asset!("/assets/icons/stack/python.svg"), name: "python", color: "#B3D3F2", link: "https://www.python.org/" },
            ],
            repository: "https://github.com/remysedlak/tkinter-audio-analysis",
            desc: "Python <b>GUI</b>, plays local <b>audio files</b> and generates plots, extracting audio features via <a target='_blank' href='https://librosa.org/doc/latest/index.html'>Librosa</a>'s Fourier transform. Features include amplitude, chroma, zcr, rms, etc.",
        },
        Project {
            title: "Audio Hub",
            img_url: asset!("/assets/snapshots/audio-hub.png"),
            stack: vec![
                StackItem { img_url: asset!("/assets/icons/stack/react.svg"),name: "react", color: "#B3ECFF", link: "https://react.dev/" },
                StackItem { img_url: asset!("/assets/icons/stack/vitejs.svg"),name: "vitejs", color: "#D6D6FF", link: "https://vitejs.dev/" },
                StackItem { img_url: asset!("/assets/icons/stack/tailwindcss.svg"),name: "tailwindcss", color: "#B2F5EA", link: "https://tailwindcss.com/" },
            ],
            repository: "https://github.com/remysedlak/audio-hub",
            desc: "This JS app is designed for <b>music producers</b> to evaluate information from <b>samples</b>. Currently developing a reliable browser-side spectrogram algorithm.",
        },
        Project {
            title: "ClipVault",
            img_url: asset!("/assets/snapshots/clipvault.png"),
            stack: vec![
                StackItem { img_url: asset!("/assets/icons/stack/rust.svg"),name: "rust", color: "#B3ECFF", link: "https://www.rust-lang.org/" },
                StackItem { img_url: asset!("/assets/icons/stack/sqlite.png"),name: "sqlite", color: "#B3ECFF", link: "https://www.sqlite.org/" },
            ],
            repository: "https://github.com/remysedlak/clipvault",
            desc: "ClipVault is a <b>lightweight</b>, <b>privacy-focused</b> clipboard manager built in <b>Rust</b> with a native GUI using <b>egui</b> and <b>system tray integration</b>.",
        },
        Project {
            title: "Plant Parent",
            img_url: asset!("/assets/snapshots/plant-mob.png"),
            stack: vec![
                StackItem { img_url: asset!("/assets/icons/stack/react.svg"),name: "react", color: "#B3ECFF", link: "https://react.dev/" },
                StackItem { img_url: asset!("/assets/icons/stack/django.svg"),name: "django", color: "#B3C6B3", link: "https://www.djangoproject.com/" },
                StackItem { img_url: asset!("/assets/icons/stack/render.svg"),name: "render", color: "#B2F5EA", link: "https://render.com/" },
            ],
            repository: "https://github.com/remysedlak/plant-parent",
            desc: "This webapp tracks <b>plants</b> and their <b>needs</b>. Currently developing a login system and a database to store plant information and images.",
        },
    ]
}
