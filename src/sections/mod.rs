//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod projects;
pub use projects::Projects;

mod timeline;
pub use timeline::Timeline;

mod links;
pub use links::Links;