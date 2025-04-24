use std::convert;

use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[derive(Clone)]
pub struct School {
    pub degree: String,
    pub year: String,
    pub location: String
}

#[derive(Clone)]
pub struct Cert {
    pub cert: String,
    pub link: String
}
#[derive(Clone)]
pub enum EducationSelection {
    EducationSchools(Vec<School>),
    EducationCerts(Vec<Cert>)

}

#[component]
pub fn Education() -> Element {
    let mut selected_index = use_signal(|| 0);

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



    
                
    let schools = vec![
                School { 
                    degree: "Masters of Science in Analytics".to_string(),
                    year: "2017".to_string(),
                    location: "Louisiana State University".to_string()
                },
                School { 
                    degree: "Bachelors of Science in Electrical Engineering".to_string(),
                    year: "2016".to_string(),
                    location: "University of New Orleans".to_string()
                },
                School { 
                    degree: "Data Engineering Nanodegree - Udacity".to_string(),
                    year: "2021".to_string(),
                    location: "Udacity".to_string()
                }
            ];

    let certs = vec! [

        Cert {
            cert: "AWS Associate Architect".to_string(),
            link: "".to_string(),
        },
        Cert {
            cert: "TensorFlow Developer Certification".to_string(),
            link: "".to_string(),
        },
        Cert {
            cert: "Mathematics for Machine Learning - Specialization - Coursera.org".to_string(),
            link: "".to_string(),
        },
        Cert {
            cert: "Kaggle Competitions Expert".to_string(),
            link: "".to_string(),
        }

    ];
            

    let sections = [
        EducationSelection::EducationSchools(schools),
        EducationSelection::EducationCerts(certs),
        ];

    let sections_clone = sections.clone();
    let sections_clone_2 = sections.clone();

    let get_section_info = use_memo(move || {
            let indx = selected_index.read().clone();
            match &sections[indx] {
                EducationSelection::EducationSchools(schools) => {
                    
                    rsx! {
                        h3 { class: "font-medium txt-dark-purple-rk", "Education" }
                    }
                }
                EducationSelection::EducationCerts(certs) => {
                    
                    rsx! {
                        h3 { class: "font-medium txt-dark-purple-rk", "Certifications & Achievements" }
                    }
            }
        }
        }
    );

    let profile_comp = sections_clone.iter().enumerate().map(|(index, (selection))| {
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
                    selected_index.set(index);
                    animate_details(());
                },
                div {
                    class: "absolute -left-[25px] top-1/4 -translate-y-1/2 w-4 h-4 rounded-full hidden md:block",
                    class: if selected_index() == index { "bg-purple-rk" } else { "bg-purple-rk" },
                    style: "transform: scale({point_transform.get_value().scale});",
                }
                match selection {
                    EducationSelection::EducationSchools(schools) => {
                        
                        rsx! {
                            div {
                                class: "p-4 rounded-lg transition-colors duration-300",
                                class: if selected_index() == index { "bg-surface-gold-rk" } else { "bg-surface-gold-rk/50" },
                                h3 { class: "font-medium txt-dark-purple-rk", "EDUCATION" }
                            }
                        }
                    }
                    EducationSelection::EducationCerts(certs) => {
                        rsx! {
                            div {
                                class: "p-4 rounded-lg transition-colors duration-300",
                                class: if selected_index() == index { "bg-surface-gold-rk" } else { "bg-surface-gold-rk/50" },
                                h3 { class: "font-medium txt-dark-purple-rk", "CERTS & ACHIEVES" }
                            }
                        }
                    }
                }
            }
        }
    });

    

    let get_experience_section = use_memo(move || {

        let indx = selected_index.read().clone();
        match &sections_clone_2[indx] {
            EducationSelection::EducationSchools(schools) => {

                rsx! {
                    div {
                        style: "margin-top: 10px;",
                        {schools.iter().enumerate().map(|(index, school)|

                            rsx! {
                                div {
                                    h3 { class: "text-xl font-semibold txt-gold-rk", "{school.degree}" }
                                    p { class: "txt-gray-blue-rk", "{school.year}" }
                                    p { class: "txt-gray-blue-rk", "{school.location}" }
                                }
                                br {}
                            }
                            )
                        }
                    }
                }
            }
            EducationSelection::EducationCerts(certs) => {
                
                rsx! {

                    div {
                        style: "margin-top: 10px;",
                    
                    {
                        certs.iter().enumerate().map(|(index, cert)|

                        rsx! {
                            div { class: "space-y-4",
                                h3 { class: "text-xl font-semibold txt-gold-rk", "{cert.cert}" }
                            }
                            br {}
                        }
                    )}
                    }
                }
        }
    }

    });

    
    rsx! {
        div { id: "experience", class: "container mx-auto px-4 py-12 overflow-x-hidden",
            h2 { class: "text-3xl font-bold bg-clip-text text-transparent bg-purple-rk mb-8",
                "Education & Achievments"
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
                div { class: "w-full md:w-1/4 space-y-8 pl-4", {profile_comp} }
                div {
                    id: "experience-details",
                    class: "w-full md:w-3/4 bg-dark-purple-rk rounded-xl p-8 border border-gray-800",
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

                        {get_experience_section()}
                }
            }
        }
    }
}