use dioxus::prelude::*;

use crate::routes::Route;
use crate::server_fns::curriculum::get_node;

#[component]
pub fn NodeView(id: String) -> Element {
    let mut node_data = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| true);
    let mut error_msg = use_signal(|| None::<String>);

    let id_clone = id.clone();
    use_effect(move || {
        let id = id_clone.clone();
        spawn(async move {
            match get_node(id).await {
                Ok(data) => {
                    node_data.set(Some(data));
                    loading.set(false);
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to load node: {}", e)));
                    loading.set(false);
                }
            }
        });
    });

    rsx! {
        div { class: "container",
            if *loading.read() {
                p { "Loading..." }
            } else if let Some(err) = &*error_msg.read() {
                p { class: "error", "{err}" }
            } else if let Some(data) = &*node_data.read() {
                div { class: "node",
                    // Back to curriculum
                    if let Some(curriculum_id) = data["curriculum_id"].as_str() {
                        Link {
                            to: Route::CurriculumView { id: curriculum_id.to_string() },
                            class: "back-link",
                            "← Back to Curriculum"
                        }
                    }

                    h1 { {data["title"].as_str().unwrap_or("Untitled")} }

                    if let Some(summary) = data["summary"].as_str() {
                        if !summary.is_empty() {
                            p { class: "summary", "{summary}" }
                        }
                    }

                    // Content
                    if let Some(content) = data["content"].as_str() {
                        if !content.is_empty() {
                            div { class: "content",
                                // For Phase 1, render content as plain text
                                // Phase 2 will add markdown rendering
                                pre { "{content}" }
                            }
                        }
                    }

                    // Action buttons (Phase 2 will wire these up)
                    div { class: "actions",
                        h3 { "Explore" }
                        div { class: "action-buttons",
                            button { class: "action deeper", disabled: true,
                                "Dive Deeper"
                            }
                            button { class: "action siblings", disabled: true,
                                "More Like This"
                            }
                            button { class: "action next", disabled: true,
                                "Move On"
                            }
                        }
                    }

                    // Next edges
                    if let Some(next_nodes) = data["edges"]["next"].as_array() {
                        if !next_nodes.is_empty() {
                            div { class: "edge-list",
                                h3 { "Next" }
                                for node in next_nodes.iter() {
                                    Link {
                                        to: Route::NodeView {
                                            id: node["id"].as_str().unwrap_or("").to_string(),
                                        },
                                        class: "edge-card",
                                        {node["title"].as_str().unwrap_or("Untitled")}
                                    }
                                }
                            }
                        }
                    }

                    // Children
                    if let Some(children) = data["children"].as_array() {
                        if !children.is_empty() {
                            div { class: "children-list",
                                h3 { "Sub-topics" }
                                for child in children.iter() {
                                    Link {
                                        to: Route::NodeView {
                                            id: child["id"].as_str().unwrap_or("").to_string(),
                                        },
                                        class: "child-card",
                                        {child["title"].as_str().unwrap_or("Untitled")}
                                    }
                                }
                            }
                        }
                    }

                    // Videos (Phase 4 placeholder)
                    if let Some(videos) = data["videos"].as_array() {
                        if !videos.is_empty() {
                            div { class: "videos",
                                h3 { "Videos" }
                                for video in videos.iter() {
                                    div { class: "video-card",
                                        a {
                                            href: "https://youtube.com/watch?v={video_id}",
                                            target: "_blank",
                                            {video["title"].as_str().unwrap_or("Video")}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
