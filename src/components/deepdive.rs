use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use std::time::Duration;

#[component]
pub fn DeepDiveBlogList() -> Element {
    let blog_posts = [
        ("My Reflections on Tech & AI in 2024", "Notes on My Personal Growth and Opinions on the Benefits of GenAI", "https://medium.com/init-deep-dive/my-reflections-on-tech-ai-in-2024-f7a6fcf7b814"),
        ("[LangGraph] Self-Correcting Code Assistant + Rag Search", "AI Agent for Code Generation with RAG + Self-Reflection", "https://medium.com/init-deep-dive/langgraph-self-correcting-code-assistant-rag-search-74b575d02505"),
        ("LinkedIn Easy-Apply Bot with ChatGPT 4o and LangGraph Agent", "Web Automation and Data Collection", "https://medium.com/init-deep-dive/linkedin-easy-apply-bot-with-chatgpt-4o-and-langgraph-agent-4ab9f50e017c"),
        ("[Python] Finetune any Open-Source LLM as a Re-Ranker", "Pairwise Ranking Prompting Strategy for Open-Source and API-Based LLMs", "https://medium.com/init-deep-dive/python-finetune-any-open-source-llm-as-a-re-ranker-d23731e5385b"),
        ("Voice Cloning with Tacotron2 and HiFi-GAN", "Fine-tuning Text-Wav and Wav-Speech models", "https://medium.com/init-deep-dive/voice-cloning-with-tacotron2-and-hifi-gan-30193180be32"),
        ("[AWS] Creating a Transactions Database with Bare Bones AWS Services for a Hypothetical Web App", "Integrating a Web App with DynamoDB, API Gateway, and Cognito", "https://medium.com/init-deep-dive/full-stack-creating-a-transactions-database-with-bare-bones-aws-services-for-a-hypothetical-web-6848421404dc"),
        ("Image Hashing Networks with Metric Learning", "Inspired by Apple’s CSAM Image Hashing Algorithm", "https://medium.com/init-deep-dive/image-hashing-networks-with-metric-learning-3fc937a1348d"),
        ("[Rust] Serving Llama 3 Quantized (GGUF), on GPU with Candle-RS and Tide", "Serving Quantized Llama 3 Model on a GPU on Windows 11", "https://medium.com/init-deep-dive/rust-quantized-llama-3-gguf-on-a-gpu-with-candle-rs-cecadd083aec"),
        ("Testing AWS Lambda with Docker Golang on Windows 11 Locally", "The setup for testing AWS Golang Lambda locally on 7/17/2024", "https://medium.com/init-deep-dive/testing-aws-docker-golang-lambda-locally-on-windows-11-1403d3d1067a"),
        ("[Rust] Video Frame Extraction Speed Comparison", "FFMPEG-Next vs OpenCV vs Video-RS", "https://medium.com/init-deep-dive/rust-video-frame-extraction-speed-comparison-4d33fcc99405"),
        ("Setting up a Dev Container on Windows 11 for Rust Development", "LLVM, FFMPEG, and OpenCV on Windows 11 in a Dev Container", "https://medium.com/init-deep-dive/setting-up-a-dev-container-on-windows-11-for-rust-development-83038cc11af3"),
        ("Advanced RAG Cookbook", "Code Examples and Practical Use Cases for Advanced RAG", "https://medium.com/init-deep-dive/advanced-rag-cookbook-b0f8db0b3b59"),
        ("[Rust] Comparing Rustformers and Candle for LLM Inference", "Running Meta/Llama Models Locally", "https://medium.com/init-deep-dive/rust-comparing-rustformers-and-candle-for-llm-inference-88e1bd4c49fe"),
        ("Creating an LLM Web App — GPT Researcher", "I hesitated a whole week to write this post because 50% of the tech stack in this post was learned, tested, and deployed in around a month…", "https://medium.com/init-deep-dive/creating-an-llm-web-app-gpt-researcher-239bd8dd92a9"),
        ("Building a CI/CD Pipeline for a Flask-React OpenAI Web App with AWS Golang CDK", "This article combines and modifies the AWS Go CDK “codepipeline-build-deploy” example and a “react-flask-app” example to create a simple…", "https://medium.com/init-deep-dive/building-a-ci-cd-pipeline-for-a-flask-react-openai-web-app-with-aws-golang-cdk-b69828eb4d36"),
        ("API Speed Tests of Python-Rust Extensions in AWS Lambda", "Rust is a new-ish programming language that has the potential to displace other programming language in the data engineering space. This is…", "https://medium.com/init-deep-dive/api-speed-tests-of-python-rust-extensions-in-aws-lambda-f5d48599385b"),
        ("Simple HuggingFace YouTube Summarizer Pipeline and Athena DB (HuggingFace/OpenAI),", "Google Maps", "https://medium.com/init-deep-dive/simple-huggingface-youtube-summarizer-pipeline-and-athena-db-huggingface-openai-742298d26865"),
        ("Creating A Real-Time (Fine-Tuned), GPT-2 Chatbot in World of Warcraft", "I created a “fine-tuned” chat bot, with GPT-2, to responded to players in a video game called world of warcraft (WoW),. Why am I doing this…", "https://medium.com/init-deep-dive/creating-a-real-time-gpt-2-chatbot-in-world-of-warcraft-958609f60f3f"),
        ("Reinforcement Learning to Approximate Mixed-Integer Allocation Solutions", "Training a Neural Network with Reinforcement Learning on Simulated Data", "https://medium.com/init-deep-dive/reinforcement-learning-to-approximate-mixed-integer-allocation-solutions-on-simulated-data-ce8f770fdb5b"),
        ("Time Series Aggregations with Core PySpark", "Moving window aggregation strategies with core PySpark and visualizations with Plotly", "https://medium.com/init-deep-dive/neat-time-series-aggregations-with-core-pyspark-4a739953076a"),
        ("Pixel Shift Motion Correction — iOS", "The objective is to implement a lateral movement motion correction algorithm on a iOS device. The algorithm processes the video feed in…", "https://medium.com/init-deep-dive/pixel-shift-motion-correction-ios-a2389785a3f5"),
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
                "Technical deep dives into software engineering, data engineering, and data science."
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

#[derive(Clone, PartialEq, Props)]
pub struct ExpandableBarProps {
    items: Vec<(String, String, String)>, // (title, subtitle, url)
}

#[component]
pub fn ExpandableBar(props: ExpandableBarProps) -> Element {
    // State to control the expansion of the bar
    let mut is_expanded = use_signal(|| false);

    // Animation for the bar's height
    let mut bar_height = use_motion(0.0f32);
    let mut bar_opacity = use_motion(0.0f32);

    // Animation for the container's border and shadow
    let mut container_transform = use_motion(Transform::new(0.0, 20.0, 0.9, 0.0));

    // Calculate dynamic height based on number of items (approximate)
    let item_height = 65.0; // Approximate height per item
    let padding = 48.0; // Account for padding (p-6 = 24px top + bottom)
    let target_height = if props.items.is_empty() {
        2.0
    } else {
        (props.items.len() as f32 * item_height) + padding
    };

    // Toggle expansion and animate
    let toggle_expansion = move |_| {
        let expand_status = *is_expanded.read();
        is_expanded.set(!expand_status);
        let target_height = if *is_expanded.read() { target_height } else { 0.0 };
        let target_opacity = if *is_expanded.read() { 1.0 } else { 0.0 };

        bar_height.animate_to(
            target_height,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 15.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );

        bar_opacity.animate_to(
            target_opacity,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(400),
                easing: easer::functions::Sine::ease_in_out,
            })),
        );

        container_transform.animate_sequence(
            AnimationSequence::new().then(
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 120.0,
                    damping: 12.0,
                    mass: 1.0,
                    velocity: 0.0,
                }))
                .with_delay(Duration::from_millis(200)),
            ),
        );
    };

    rsx! {
        div {
            class: "px-4 py-6",
            // Header and toggle button
            div {
                class: "flex justify-between items-center mb-4",
                button {
                    onclick: toggle_expansion,
                    class: "px-6 py-3 text-xl rounded-full bg-dark-purple-rk bg-gray-800 text-text-secondary text-gray-300 hover:bg-primary hover:bg-blue-600 hover:text-dark-purple-rk hover:text-gray-900 transition-colors duration-300",
                    if *is_expanded.read() { "Collapse" } else { "Expand" }
                }
            }
            // Expandable content
            div {
                class: "rounded-xl bg-dark-purple-rk border border-surface-light/20 transition-all duration-300 hover:border-surface-light/40 hover:shadow-xl hover:shadow-primary/20",
                style: "height: {bar_height.get_value()}px; transform: translateY({container_transform.get_value().y}px) scale({container_transform.get_value().scale}); opacity: {bar_opacity.get_value()};",
                div {
                    class: "p-6",
                    for (index, (title, subtitle, url)) in props.items.iter().enumerate() {
                        LinkItem {
                            key: "{index}", // Ensures stable rendering
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
            style: "",
            a {
                href: "{props.url}",
                target: "_blank",
                h3 {
                    class: "text-lg font-semibold txt-gold-rk hover:text-primary transition-colors",
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