use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use std::time::Duration;

#[component]
pub fn BlogList() -> Element {
    let blog_posts = [
        ("Init Deep Dive", "Technical deep dives into software engineering, data engineering, and data science.", "https://medium.com/init-deep-dive"),
        ("Lazy By Design", "A collection of applied data science stories & projects.", "https://medium.com/lazy-by-design"),
        ("Intro Zero", "Introductory programming and math content", "https://medium.com/intro-zero")
    ];

    rsx! {
        div {
            id: "initdeepdive",
            class: "container mx-auto px-4 py-12",
            // Section header
            h2 {
                class: "text-3xl font-bold bg-clip-text txt-purple-rk mb-2",
                "Technical Blog Posts"
            }
            p {
                class: "txt-purple-rk mb-8",
                "Publications of blog posts maintained by me."
            }
            ExpandableBar {
                items: blog_posts
                    .into_iter()
                    .map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string()))
                    .collect()
            }
        }
    }
}

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ExpandableBarProps {
    items: Vec<(String, String, String)>, // (title, subtitle, url)
}

#[component]
pub fn ExpandableBar(props: ExpandableBarProps) -> Element {
    // State to control the expansion of the bar
    let mut is_expanded = use_signal(|| false);

    // Toggle expansion
    let toggle_expansion = move |_| {
        let expand_status = *is_expanded.read();
        is_expanded.set(!expand_status);
        println!("Toggling: is_expanded = {}", *is_expanded.read());
    };

    rsx! {
        div {
            class: "deepdiveexpandable-bar",
            // Header and toggle button
            div {
                class: "deepdiveheader",
                h2 {
                    class: "deepdiveheader-title p-4",
                    "Resource Links"
                }
                button {
                    class: "deepdivetoggle-button",
                    onclick: toggle_expansion,
                    if *is_expanded.read() { "Collapse" } else { "Expand" }
                }
            }
            // Expandable content
            div {
                class: "deepdiveexpandable-content",
                style: if *is_expanded.read() { "max-height: 500px;" } else { "max-height: 0px;" },
                div {
                    class: "deepdivecontent-inner",
                    if props.items.is_empty() {
                        p { class: "empty-message", "No items to display" }
                    } else {
                        for (index, (title, subtitle, url)) in props.items.iter().enumerate() {
                            LinkItem {
                                key: "{index}",
                                title: title.clone(),
                                subtitle: subtitle.clone(),
                                url: url.clone(),
                                index: index,
                                is_visible: *is_expanded.read()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct LinkItemProps {
    title: String,
    subtitle: String,
    url: String,
    index: usize,
    is_visible: bool,
}

#[component]
fn LinkItem(props: LinkItemProps) -> Element {
    // Animations for each item
    rsx! {
        div {
            class: "mb-4 last:mb-0",
            style: if props.is_visible { "display:block; max-height: 100px; overflow: hidden;" } else { "display: none; max-height: 65px; overflow: hidden;"},
            a {
                href: "{props.url}",
                target: "_blank",
                h3 {
                    class: "text-lg font-semibold text-accent-purple hover:text-accent-purple-hover transition-colors",
                    "{props.title}"
                }
                p {
                    class: "text-sm txt-gray-blue-rk hover:text-primary transition-colors",
                    "{props.subtitle}"
                }
            }
        }
    }
}