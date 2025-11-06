#![allow(non_snake_case)]

use components::{NavBar, Profile, ProjectGrid, WorkExperience, BlogList, Education};
use dioxus::prelude::*;

use views::{Blog, BlogPreview, Footer};
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: i32 },
}
pub const MARKDOWN_CSS: Asset = asset!("/assets/markdown.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const PROFILE_PIC: Asset = asset!("/assets/1152300.png");
pub const MOTION_PIC: Asset = asset!("/assets/dioxus-motion.png");
pub const HTML_RSX_PIC: Asset = asset!("/assets/html-rsx.png");
pub const STYLES_CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        document::Title { "Rohan Kotwani" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css",
        }
        
        document::Link {
            rel: "stylesheet",
            href: STYLES_CSS,
        }

        Router::<Route> {}
    }
}

pub static PROFILE_ELEMENT: GlobalSignal<
    Option<dioxus::prelude::Event<dioxus::events::MountedData>>,
> = Global::new(|| None);
pub static WORKEXPERIENCE: GlobalSignal<
    Option<dioxus::prelude::Event<dioxus::events::MountedData>>,
> = Global::new(|| None);
pub static PROJECT_GRID: GlobalSignal<Option<dioxus::prelude::Event<dioxus::events::MountedData>>> =
    Global::new(|| None);
pub static BLOG_PREVIEW: GlobalSignal<Option<dioxus::prelude::Event<dioxus::events::MountedData>>> =
    Global::new(|| None);

pub static EDUCATION: GlobalSignal<Option<dioxus::prelude::Event<dioxus::events::MountedData>>> =
    Global::new(|| None);

// Home component - Main landing page container
#[component]
fn Home() -> Element {
    let css = MAIN_CSS.to_string();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        div {

           class: "min-h-screen bg-background text-text-primary",

            // Main container
            // Hero/Profile section
            onmounted: move |data| {
                *PROFILE_ELEMENT.write() = Some(data);
            },
            Profile {
            }

            // Work Experience section
            div {
                onmounted: move |data| {
                    *WORKEXPERIENCE.write() = Some(data);
                },
                WorkExperience {}
            }

            div {
                onmounted: move |data| {
                    *EDUCATION.write() = Some(data);
                },
                Education {}

            }

            // Project Grid
            div {
                onmounted: move |data| {
                    *PROJECT_GRID.write() = Some(data);
                },
                ProjectGrid {}
            }

            // Blog Preview
            div {
                onmounted: move |data| {
                    *BLOG_PREVIEW.write() = Some(data);
                },
                BlogList {}
            }
            // Footer
            div { Footer {} }
        }
    }
}
