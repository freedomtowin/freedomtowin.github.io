use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn WorkExperience() -> Element {
    let mut selected_company = use_signal(|| 0);

    let mut dot_transform = use_motion(Transform::identity());
    let mut details_transform = use_motion(Transform::identity());

    let mut animate_details = move |_| {
        details_transform.animate_to(
            Transform::new(0.0, 0.0, 1.0, 0.0),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 15.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    let companies = [
        (
            "FOOLPROOF STRATEGIES",
            "2024 - 2025",
            "AI/GOLANG/RUST ENGINEER",
            "* Created over 200 articles/videos using AI-assisted content generation tools.\n* Built scalable serverless pipelines to process and transform data from sources like videos, webpages, and PDFs.\n* Fine-tuned models for text generation, chat completion, reranking, and text-to-speech.\n* Served AI models and RAG through REST API, SOCKET,\n* Built scalable infrastructure with Golang using AWS CDK, SDK, and Lambda\n* Built AI Agents with LangGraph for a variety of tasks.\n* Cross platform app development with Rust",
            vec!["Rust", "Python", "AWS", "Golang", "Docker", "Dioxus", "LangGraph"],
        ),
        (
            "CORE COMPETE",
            "2022 - 2024",
            "DATA ENGINEER",
            "CAPITAL ONE\nSupported Capital One's Data Product team to develop a Schema comparison tool to track discrepancies for schemas throughout the data pipeline from tech teams to downstream consumer data interfaces: Logical Data Models, Enterprise Dictionaries, APIs, and GraphQL servers.\n\nSEMPRA ENERGY\nSupported Sempra's Wildfire Risk team to migrate ETL jobs from proof-of-concept to AWS production. Created Glue jobs, SageMaker scripts, Step Functions, Lambda endpoints, Athena Tables, and Terraform scripts for dev, test, and production environments.\n\nGAP INC\nBuilt an arbitration optimization model (PuLP, Gurobi, VSCode, Pytest, Github) that distributed inventory from the distribution centers to stores. Developed business scenario testing & data\\algorithm unit testing. Supported data engineering (Azure Data Factory\\DataBricks) by migrating data, building data pipelines (PySpark), and exporting data (CLI) for DS model testing.",
            vec!["Python", "Gurobi", "AWS", "Databricks", "SQL"],
        ),
        (
            "NATIONAL TARGETING CENTER",
            "2018 - 2021",
            "DATA SCIENTIST",
            "Deployed two models with a large operational impact, and several auxiliary models, while supporting Customs and Border Protection on projects regarding the facilitation and security of cargo and passengers.\n\nWorked on all stages of machine learning model deployment, i.e., data engineering (SQL, python), concept-based data mining (Python), model development (TensorFlow, GBM), and deployment validation (Java, gRPC).\n\nEngaged with the client, the National Target Center, weekly, to present metrics, updates, and to gather new information. Collaborated with data engineers to understand and deploy models in a real-time complex environment.\n\n1) Extreme class-imbalanced classification 2) Anomaly detection Markov chains implementation and simulation 3) Entity resolution 4) Graph network utilization in predictive models 5) NLP utilization in predictive model\\product classification 6) Geospatial encoding 7) Open-source data collection.",
            vec!["Python", "PyTorch", "SQL"],
        ),
    ];

    let companies_comp = companies.iter().enumerate().map(|(index, (company, duration, _, _, _))| {
        let mut point_transform = use_motion(Transform::identity());

        let animate_hover = move |_| {
            point_transform.animate_to(
                Transform::new(0.0, 0.0, 1.2, 0.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 180.0,
                    damping: 12.0,
                    mass: 1.0,
                    ..Default::default()
                })),
            );
        };

        let animate_reset = move |_| {
            point_transform.animate_to(
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );
        };

        rsx! {
            div {
                class: "relative cursor-pointer group",
                onmouseenter: animate_hover,
                onmouseleave: animate_reset,
                onclick: move |_| {
                    selected_company.set(index);
                    animate_details(());
                },
                div {
                    class: "absolute -left-[25px] top-1/4 -translate-y-1/2 w-4 h-4 rounded-full hidden md:block",
                    class: if selected_company() == index { "bg-purple-rk" } else { "bg-purple-rk" },
                    style: "transform: scale({point_transform.get_value().scale});",
                }
                div {
                    class: "p-4 rounded-lg transition-colors duration-300",
                    class: if selected_company() == index { "bg-surface-gold-rk" } else { "bg-surface-gold-rk/50" },
                    h3 { class: "font-medium txt-dark-purple-rk", "{company}" }
                    p { class: "text-sm txt-dark-purple-rk", "{duration}" }
                }
            }
        }
    });

    let experience_section = {
        let (_company, duration, title, description, tech_stack) = &companies[selected_company()];
        let description_split = description.split("\n");
        rsx! {
            div { class: "space-y-4",
                h3 { class: "text-xl font-semibold txt-gold-rk", "{title}" }
                p { class: "txt-gray-blue-rk", "{duration}" }
                {description_split.into_iter().map(|content| {
                    rsx! { p { class: "txt-gray-blue-rk leading-relaxed", "{content}" } }
                })}
                div { class: "flex flex-wrap gap-2",
                    {tech_stack.iter().enumerate().map(|(index, tech)| {
                        let mut tech_transform = use_motion(Transform {
                            scale: 0.8,
                            y: 10.0,
                            x: (index as f32 * 10.0),
                            rotation: 0.0,
                        });
                        rsx! {
                            span {
                                class: "px-3 py-1 text-xs rounded-full bg-pruple-rk txt-gold-rk",
                                style: "transform: translate({tech_transform.get_value().x}px, {tech_transform.get_value().y}px) scale({tech_transform.get_value().scale}) rotate({tech_transform.get_value().rotation}deg);",
                                onmounted: move |_| async move {
                                    let delay = Duration::from_millis((200 + index as u64 * 100));
                                    Time::delay(delay).await;
                                    tech_transform.animate_to(
                                        Transform::identity(),
                                        AnimationConfig::new(AnimationMode::Spring(Spring {
                                            stiffness: 100.0,
                                            damping: 15.0,
                                            mass: 1.0,
                                            velocity: 0.0,
                                        })),
                                    );
                                },
                                "{tech}"
                            }
                        }
                    })}
                }
            }
        }
    };

    rsx! {
        div { id: "experience", class: "container mx-auto px-4 py-12 overflow-x-hidden",
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-purple-rk mb-8",
                "Work Experience"
            }
            div { class: "flex flex-col md:flex-row gap-6 md:gap-8",
                div {
                    id: "timeline-line",
                    class: "relative w-1 bg-gray-800 hidden md:block",
                    div {
                        id: "timeline-dot",
                        class: "absolute w-2 h-2 bg-blue-500 rounded-full -left-[3px]",
                        style: "transform: translateY({dot_transform.get_value().y}px);",
                        onmounted: move |_| {
                            dot_transform.animate_to(
                                Transform::new(0.0, 200.0, 1.0, 0.0),
                                AnimationConfig::new(AnimationMode::Spring(Spring {
                                    stiffness: 100.0,
                                    damping: 15.0,
                                    mass: 1.0,
                                    velocity: 10.0,
                                })).with_loop(LoopMode::Infinite),
                            );
                        },
                    }
                }
                div { class: "w-full md:w-1/4 space-y-8 pl-4", {companies_comp} }
                div {
                    id: "experience-details",
                    class: "w-full md:w-3/4 bg-dark-purple-rk rounded-xl p-6 border border-gray-800",
                    style: "transform: scale({details_transform.get_value().scale});",
                    onmounted: move |_| {
                        details_transform.animate_to(
                            Transform::new(0.0, 0.0, 1.0, 0.0),
                            AnimationConfig::new(AnimationMode::Spring(Spring {
                                stiffness: 100.0,
                                damping: 15.0,
                                mass: 1.0,
                                velocity: 0.0,
                            })),
                        );
                    },
                    {experience_section}
                }
            }
        }
    }
}