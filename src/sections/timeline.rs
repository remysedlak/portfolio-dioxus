use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TimeLineItem {
    pub details: &'static str,
    pub title: &'static str,
    pub duration: &'static str,
    pub year: &'static str,
}

pub const TIMELINE: &[TimeLineItem] = &[
    TimeLineItem {
        year: "2025",
        title: "Return @ AGI",
        duration: "present",
        details: "This summer, I got the opportunity to return to <b>AssetGenie</b> as a developer intern, where I am currently strategizing and implementing <b>database integration</b> between local and cloud ERP systems. More details to come!",
    },
    TimeLineItem {
        year: "2025",
        title: "H4H Finals",
        duration: "1 month",
        details: "Continuing our hackathon project from February, Devon and I were honored to present our <a target=\"_blank\" href=\"https://github.com/DSmith215/The-Inquistor\">Deepfake Detector</a> in Harrisburg to a panel of judges, including Pennsylvania’s <b>First Lady</b>, Lori Shapiro. The governor’s residence was <b>breathtaking</b>, and the event emphasized the growing importance of <b>AI ethics</b> in real-world applications.",
    },
    TimeLineItem {
        year: "2025",
        title: "Tutoring",
        duration: "5 months",
        details: "During my sophomore year, I started providing one-on-one <b>tutoring sessions</b> for <a target=\"_blank\" href=\"https://courses.sci.pitt.edu/courses\">SCI courses</a> at Pitt, guiding students through fundamental and advanced programming concepts. This opportunity has allowed me to strengthen students' <b>problem-solving</b> and debug skills, while I strengthen my own <b>communication</b>.",
    },
    TimeLineItem {
        year: "2025",
        title: "Hackathon #1",
        duration: "2 weeks",
        details: "I competed in <a target=\"_blank\" href=\"https://www.duq.edu/research/centers-and-institutes/grefenstette-center/hacking4humanity.php\">Hacking4Humanity 2025</a>, a hackathon focused on <b>AI for social good</b>, where I collaborated with a team to develop an AI-powered <b>deepfake detection</b> model. The project earned an <b>Honorable Mention</b>, and this experience deepened my passion for <b>AI ethics</b>, adversarial machine learning, and responsible AI development.",
    },
    TimeLineItem {
        year: "2024",
        title: "Intern @ AGI",
        duration: "2 months",
        details: "Secured my first <b>technical internship</b> at <a target=\"_blank\" class=\"underline text-blue-400\" href=\"https://agigrouponline.com/\">AssetGenie Inc.</a>, where I built <b>Python automation tools</b> integrated with GitLab and Excel to streamline data processing workflows. Gained hands-on experience in <b>data engineering</b>, scripting, and backend development, enhancing operational efficiency across teams. Worked in a <b>fast-paced environment</b>, collaborating with engineers to optimize automation processes and improve internal data management systems.",
    },
    TimeLineItem {
        year: "2023",
        title: "CS @ Pitt",
        duration: "4 years",
        details: "Began my persuit of a <b>B.S. in Computer Science</b> and <b>Digital Narrative and Interactive Design</b> at the <a target=\"_blank\" href=\"https://www.pitt.edu/\">University of Pittsburgh</a>, with a focus on the intersection of <b>technology</b> and human behavior. Applying <b>interdisciplinary knowledge</b> to real-world projects in AI, UX design, and music technology.",
    },
    TimeLineItem {
        year: "2022",
        title: "Warehousing",
        duration: "2 years",
        details: "Worked as a <b>warehouse associate</b> at <a target=\"_blank\" href=\"https://www.fayettepartsservice.com/\">Fayette Service Parts Inc.</a>, managing <b>inventory</b> for over 25+ NAPA automotive stores. Gained experience in <b>logistics</b>, supply chain operations, and data tracking, ensuring accurate inventory management in a high-demand environment. Developed strong <b>problem-solving skills</b> by troubleshooting logistical challenges and optimizing warehouse efficiency.",
    },
    TimeLineItem {
        year: "2021",
        title: "Hello World!",
        duration: "9 months",
        details: "Took my first <b>programming course</b> through a <a target=\"_blank\" href=\"https://www.chs.pitt.edu/\">College in High School (CHS)</a> program, sparking my passion for <b>software development</b>. Started with <b>fundamental programming concepts</b> and quickly became fascinated with problem-solving, leading to deeper explorations in AI, full-stack development, and data science. This moment marked the <b>foundation</b> of my journey into technology, setting the stage for future projects and innovations.",
    },
];

#[component]
pub fn Timeline() -> Element {
    rsx! {
        div { id:"timeline",
            
            class: "p-8 space-y-6 bg-slate-200",
            h1 {
                class:"text-4xl md:text-4xl font-bold mb-5",
                "Timeline"
            }
            div {
                class: "flex flex-col gap-8 pb-8",
                {TIMELINE.iter().map(|entry| {
                    rsx! {
                        div {
                            class: "bg-gray-100 rounded-2xl p-6 shadow-lg",
                            h2 {
                                class: "text-2xl font-semibold",
                                "{entry.title} ({entry.year})"
                            }
                            p {
                                class: "text-sm text-gray-500 italic mb-2",
                                "{entry.duration}"
                            }
                            div {
                                class: "text-base text-gray-700",
                                dangerous_inner_html: "{entry.details}"
                            }
                        }
                    }
                })}
            }
        }
    }
}
