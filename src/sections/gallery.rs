use dioxus::prelude::*;
use gloo_timers::callback::Interval;

#[component]
pub fn About() -> Element {
    let images = vec![
        asset!("/assets/slideshow/h4h-2025-204-2.jpg"),
        asset!("/assets/slideshow/54413401962_2192bd17d1_c.jpg"),
        asset!("/assets/slideshow/IMG_2863.png"),
        asset!("/assets/slideshow/IMG_2927.png"),
        asset!("/assets/slideshow/remysedlak.jpg")
    ];

    let titles = vec![
        "H4H Team, The Inquisitor",
        "H4H Finals @ Harrisburg",
        "Building my first computer",
        "My cat committing a push",
        "Portrait of Remy Sedlak"
    ];

    let mut current_index = use_signal(|| 0);
    let mut _interval = use_signal(|| None::<Interval>);
    let image_count = images.len();

    use_effect(move || {
        let mut current_index = current_index.clone();
        let mut interval_signal = _interval.clone();

        let interval = Interval::new(7000, move || {
            let current = *current_index.read();
            current_index.set((current + 1) % image_count);
        });

        interval_signal.set(Some(interval));
    });

    rsx! {
        section {
            id: "about",
            class: "p-4 md:p-8 bg-slate-200",
            div { class:" pb-8",

            
            h2 {
                class: "text-4xl text-left font-bold text-gray-800 mb-4 md:mb-6",
                "About Me"
            }
            div {
                class: "flex flex-col md:flex-row gap-y-8 flex-1 justify-around",

                // Text block - 2/3 on medium+ screens
                div {
                    class: "bg-white p-4 rounded-2xl shadow-2xl text-slate-800 space-y-6 leading-relaxed",

                    p {
                        class: "text-3xl font-bold text-slate-900 mb-4",
                        "Hey there! 👋 Thanks for visiting my page!"
                    }

                    p {
                        class: "text-xl font-medium",
                        "I'm Remy Sedlak—a builder, problem-solver, and explorer at heart."
                    }

                    p {
                        class: "text-base md:text-lg font-normal text-slate-700",
                        "I'm passionate about empowering creativity through code—whether that's crafting intuitive user experiences, building robust backend systems, or experimenting with new tools and workflows."
                    }

                    p {
                        class: "text-base md:text-lg font-normal text-slate-700",
                        "I love taking on new challenges and learning from every project I tackle. Each line of code I write is a step toward turning imagination into something real and impactful."
                    }
                }

                
            }
        }
        }
    }
}
