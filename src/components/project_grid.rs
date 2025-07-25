use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use std::time::Duration;

const TARIFF_TRACKER: Asset = asset!("/assets/tariff_tracker.png");
const EASY_APPLY_BOT: Asset = asset!("/assets/easy_apply_bot.png");
const FAT_CHANCE_EDITOR: Asset = asset!("/assets/fat_chance_editor.png");
const VOICE_CLONE: Asset = asset!("/assets/voice_clone_cover.png");
const FAQ_SEARCH: Asset = asset!("/assets/faq_search.png");

#[component]
pub fn ProjectGrid() -> Element {
    rsx! {
        div { id: "projects", class: "container mx-auto px-4 py-12",
            // Section header
            h2 { class: "text-3xl font-bold bg-clip-text txt-purple-rk mb-2",
                "Application Development"
            }
            p { class: "txt-purple-rk mb-8",
                "A collection of live documents that track my development progress in various applications."
            }
            // Project grid with fixed card sizes
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                ProjectCard {
                    title: "Custom RAG FAQ Search, an AI Power Search Engine",
                    description: "An AI powered search engine in 100% Rust. A custom RAG powered search engine that 1) runs in WASM, or within the browser's memory and 2) produces good to decent results.",
                    image: FAQ_SEARCH,
                    tech_stack: vec!["Rust", "PyTorch", "Dioxus", "AWS"],
                    link: "https://medium.com/lazy-by-design/dioxus-faq-search-an-ai-power-search-engine-c7173d92d626",
                },
                
                ProjectCard {
                    title: "[Rust, Dioxus, AWS] Tariff Tracker Web App",
                    description: "A web app tariff tracker will collects new news articles daily, aggregate weekly and summarizes them, then serve them in a protected web app with authentication.",
                    image: TARIFF_TRACKER,
                    tech_stack: vec!["Rust", "Dioxus", "Golang", "AWS"],
                    link: "https://medium.com/lazy-by-design/tariff-tracker-rust-web-app-6f5192f93f88",
                }
                ProjectCard {
                    title: "[ChatGPT, LangGraph] LinkedIn Easy-Apply Bot",
                    description: "An AI agent built to automatically navigate job boards with AI clicking, scrolling, typing, etc, and to ask for then record human input into a database, for when the agent is unsure.",
                    image: EASY_APPLY_BOT,
                    tech_stack: vec!["Python", "LangGraph"],
                    link: "https://medium.com/init-deep-dive/linkedin-easy-apply-bot-with-chatgpt-4o-and-langgraph-agent-4ab9f50e017c",
                },
                ProjectCard {
                    title: "“Fat Chance” Editor in Dioxus Rust",
                    description: "Fat Chance editor started as a simple file editor, but has grown to include more features like syntax highlighting, terminal commands, and AI integrations. It is built with Dioxus Rust.",
                    image: FAT_CHANCE_EDITOR,
                    tech_stack: vec!["Python", "LangGraph"],
                    link: "https://rohankotwani.medium.com/fat-chance-editor-in-dioxus-rust-efa2b6d437c0",
                },
                ProjectCard {
                    title: "Voice Cloning with Tacotron2 and HiFi-GAN",
                    description: "The strategies and training steps I used to finetune voice clones, Jason Thor Hall (Pirate Software) & Philomena & Norm Macdonald, with Tacotron2 and Hifi-Gan.",
                    image: VOICE_CLONE,
                    tech_stack: vec!["Python", "PyTorch"],
                    link: "https://medium.com/init-deep-dive/voice-cloning-with-tacotron2-and-hifi-gan-30193180be32",
                }
                
                // 
                // ProjectCard {
                //     title: "HTML to RSX Converter",
                //     description: "Convert HTML to Dioxus RSX with a single click. A simple yet powerful tool built with Rust and Dioxus.",
                //     image: "https://devpro-aceternity.vercel.app/_next/image?url=%2Fimages%2Fprojects%2Falgochurn.png&w=3840&q=75",
                //     tech_stack: vec!["Rust", "Dioxus", "Tailwind"],
                //     link: "https://wheregmis.github.io/dioxus_html_rsx/",
                // }

            }
        }
    }
}
#[derive(Props, PartialEq, Clone)]
struct ProjectCardProps {
    title: String,
    description: String,
    image: String,
    tech_stack: Vec<&'static str>,
    link: String,
}

#[component]
fn ProjectCard(props: ProjectCardProps) -> Element {
    // Card animations
    let mut card_transform = use_motion(Transform::new(0.0, 20.0, 0.8, 0.0));
    let mut card_opacity = use_motion(0.0f32);

    // Image animations
    let mut image_transform = use_motion(Transform::new(0.0, 0.0, 1.0, 0.0));
    let mut image_blur = use_motion(5.0f32);

    // Title animations
    let mut title_transform = use_motion(Transform::new(-20.0, 0.0, 0.9, 0.0));

    // Description animations
    let mut desc_transform = use_motion(Transform::new(-20.0, 0.0, 0.9, 0.0));

    // Card entrance animation when visible
    let on_card_visible = move |_| {
        let card_sequence = AnimationSequence::new().then(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 15.0,
                mass: 1.0,
                velocity: 0.0,
            }))
            .with_delay(Duration::from_millis(800)),
        );

        let title_sequence = AnimationSequence::new().then(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 120.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 0.0,
            }))
            .with_delay(Duration::from_millis(400)),
        );

        let desc_sequence = AnimationSequence::new().then(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 120.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 0.0,
            }))
            .with_delay(Duration::from_millis(500)),
        );

        card_transform.animate_sequence(card_sequence);
        title_transform.animate_sequence(title_sequence);
        desc_transform.animate_sequence(desc_sequence);

        card_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(600),
                easing: easer::functions::Sine::ease_in_out,
            })),
        );

        image_blur.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(800),
                easing: easer::functions::Cubic::ease_out,
            }))
            .with_delay(Duration::from_millis(300)),
        );
    };

    // Image hover animations
    let on_image_hover = move |_| {
        let image_sequence = AnimationSequence::new().then(
            Transform::new(0.0, 0.0, 1.1, 0.0),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 20.0,
                mass: 1.0,
                velocity: 5.0,
            })),
        );

        image_transform.animate_sequence(image_sequence);
    };

    let on_image_unhover = move |_| {
        let image_sequence = AnimationSequence::new().then(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 20.0,
                mass: 1.0,
                velocity: -5.0,
            })),
        );

        image_transform.animate_sequence(image_sequence);
    };

    rsx! {
        a { href: "{props.link}", target: "_blank",
            div {
                class: "group h-[400px] flex flex-col relative overflow-hidden rounded-xl bg-dark-purple-rk border border-surface-light/20 transition-all duration-300 hover:border-surface-light/40 hover:shadow-xl hover:shadow-primary/20",
                style: "transform: translateY({card_transform.get_value().y}px) scale({card_transform.get_value().scale}); opacity: {card_opacity.get_value()};",
                // Image container
                div {
                    onvisible: on_card_visible,
                    class: "relative h-[200px] w-full overflow-hidden",
                    onmouseenter: on_image_hover,
                    onmouseleave: on_image_unhover,
                    img {
                        class: "w-full h-full object-cover transition-transform duration-300",
                        style: "transform: scale({image_transform.get_value().scale}); filter: blur({image_blur.get_value()}px);",
                        src: "{props.image}",
                    }
                }

                // Content container
                div { class: "flex flex-col flex-1 p-6 h-[200px]",
                    h3 {
                        class: "text-lg font-semibold txt-gold-rk group-hover:text-primary transition-colors line-clamp-4 mb-2",
                        style: "transform: translateX({title_transform.get_value().x}px) scale({title_transform.get_value().scale});",
                        "{props.title}"
                    }
                    p {
                        class: "text-sm txt-gray-blue-rk line-clamp-4 mb-4",
                        style: "transform: translateX({desc_transform.get_value().x}px) scale({desc_transform.get_value().scale});",
                        "{props.description}"
                    }
                    // Tech stack
                    div { class: "mt-auto flex flex-wrap gap-2",
                        {
                            props
                                .tech_stack
                                .iter()
                                .enumerate()
                                .map(|(index, tech)| {
                                    let mut tech_transform = use_motion(Transform::new(0.0, 10.0, 0.0, 0.0));
                                    use_effect(move || {
                                        let tech_sequence = AnimationSequence::new()
                                            .then(
                                                Transform::identity(),
                                                AnimationConfig::new(
                                                        AnimationMode::Spring(Spring {
                                                            stiffness: 100.0,
                                                            damping: 15.0,
                                                            mass: 1.0,
                                                            velocity: 0.0,
                                                        }),
                                                    )
                                                    .with_delay(Duration::from_millis(600 + index as u64 * 100)),
                                            );
                                        tech_transform.animate_sequence(tech_sequence);
                                    });
                                    rsx! {
                                        span {
                                            class: "px-3 py-1 text-xs rounded-full bg-dark-purple-rk text-text-secondary",
                                            style: "transform: translateY({tech_transform.get_value().y}px) scale({tech_transform.get_value().scale}); opacity: {tech_transform.get_value().scale};",
                                            "{tech}"
                                        }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}
