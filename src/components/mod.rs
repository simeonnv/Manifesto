//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub use hero::Hero;

mod main_wrapper;
pub use main_wrapper::MainWrapper;

mod header;
pub use header::Header;

mod footer;
pub use footer::Footer;

mod github_icon;
pub use github_icon::GithubIcon;

mod font_face;
pub use font_face::FontFace;
