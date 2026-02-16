use dioxus::prelude::*;

use crate::routes::Route;
use crate::server_fns::curriculum::get_curriculum;

#[component]
pub fn CurriculumView(id: String) -> Element {
    let mut curriculum_data = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| true);
    let mut error_msg = use_signal(|| None::<String>);

    let id_clone = id.clone();
    use_effect(move || {
        let id = id_clone.clone();
        spawn(async move {
            match get_curriculum(id).await {
                Ok(data) => {
                    curriculum_data.set(Some(data));
                    loading.set(false);
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to load curriculum: {}", e)));
                    loading.set(false);
                }
            }
        });
    });

    rsx! {
        div { class: "container",
            Link { to: Route::Home {}, class: "back-link", "← Back to Home" }

            if *loading.read() {
                p { "Loading curriculum..." }
            } else if let Some(err) = &*error_msg.read() {
                p { class: "error", "{err}" }
            } else if let Some(data) = &*curriculum_data.read() {
                div { class: "curriculum",
                    h1 {
                        {data["curriculum"]["title"].as_str().unwrap_or("Untitled")}
                    }
                    p { class: "topic",
                        "Topic: "
                        {data["curriculum"]["topic"].as_str().unwrap_or("")}
                    }

                    div { class: "chapters",
                        h2 { "Chapters" }
                        if let Some(chapters) = data["chapters"].as_array() {
                            for chapter in chapters.iter() {
                                Link {
                                    to: Route::NodeView {
                                        id: chapter["id"].as_str().unwrap_or("").to_string(),
                                    },
                                    class: "chapter-card",
                                    h3 { {chapter["title"].as_str().unwrap_or("Untitled")} }
                                    p { {chapter["summary"].as_str().unwrap_or("")} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
