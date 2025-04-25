use dioxus::prelude::*;

use crate::{BLOG_PREVIEW, PROFILE_ELEMENT, PROJECT_GRID, WORKEXPERIENCE};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "w-full mt-auto bg-gradient-to-t from-purple-400 to-bg-background border-t bg-background",
            div { class: "container mx-auto px-4 py-12",
                // create a divider for the footer
                div { class: "border-b border-surface-light/50 mb-12" }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    // Logo and description

                    div { class: "space-y-4",
                        h3 { class: "text-xl font-semibold txt-black-rk",
                            "Rohan Kotwani"
                        }
                        p { class: "txt-black-rk", "I will update this section soon !!!" }
                    }
                    // Quick links
                    div { class: "space-y-4",
                        h4 { class: "txt-black-rk font-medium", "Quick Links" }
                        div { class: "flex flex-col space-y-2",
                            for link in ["About", "Projects", "Blogs"].iter() {
                                a {
                                    class: "text-sm txt-black-rk hover:text-text-primary transition-colors relative group cursor-pointer",
                                    onclick: move |_| async move {
                                        match *link {
                                            "About" => {
                                                if let Some(header) = PROFILE_ELEMENT.cloned() {
                                                    let _ = header.scroll_to(ScrollBehavior::Smooth).await;
                                                }
                                            }
                                            "Applications" => {
                                                if let Some(header) = PROJECT_GRID.cloned() {
                                                    let _ = header.scroll_to(ScrollBehavior::Smooth).await;
                                                }
                                            }
                                            "Technical Blog" => {
                                                if let Some(header) = BLOG_PREVIEW.cloned() {
                                                    let _ = header.scroll_to(ScrollBehavior::Smooth).await;
                                                }
                                            }
                                            _ => {}
                                        }
                                    },
                                    "{link}"

                                }
                            }
                        }
                    }
                    // Social links
                    div { class: "space-y-4",
                        h4 { class: "txt-black-rk font-medium", "Connect" }
                        div { class: "flex space-x-4",
                            a {
                                class: "txt-black-rk hover:text-white transition-colors",
                                href: "https://github.com/freedomtowin",
                                i { class: "fab fa-github text-xl" }
                            }
                            a {
                                class: "txt-black-rk hover:text-white transition-colors",
                                href: "https://twitter.com/freedomtowin",
                                i { class: "fab fa-twitter text-xl" }
                            }
                            a {
                                class: "txt-black-rk hover:text-white transition-colors",
                                href: "https://linkedin.com/in/freedomtowin",
                                i { class: "fab fa-linkedin text-xl" }
                            }
                        }
                    }
                }
                // Copyright
                div { class: "mt-12 pt-8 border-t border-gray-800 text-center txt-gray-blue-rk",
                    "© 2025 Rohan Kotwani"
                    div { class: "text-center txt-gray-blue-rk",
                        "Built with ❤️ using Dioxus"
                    }
                }

            }
        }
    }
}
