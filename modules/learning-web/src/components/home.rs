use dioxus::prelude::*;

use crate::routes::Route;
use crate::server_fns::curriculum::{create_curriculum, list_curricula};

#[component]
pub fn Home() -> Element {
    let mut topic = use_signal(|| String::new());
    let mut creating = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut curricula = use_signal(|| Vec::new());
    let nav = use_navigator();

    // Load existing curricula on mount
    use_effect(move || {
        spawn(async move {
            match list_curricula().await {
                Ok(list) => curricula.set(list),
                Err(e) => tracing::error!("Failed to load curricula: {}", e),
            }
        });
    });

    let on_submit = move |_| {
        let topic_val = topic.read().trim().to_string();
        if topic_val.is_empty() {
            return;
        }

        creating.set(true);
        error_msg.set(None);
        let nav = nav.clone();

        spawn(async move {
            match create_curriculum(topic_val).await {
                Ok(resp) => {
                    creating.set(false);
                    nav.push(Route::CurriculumView {
                        id: resp.curriculum_id,
                    });
                }
                Err(e) => {
                    creating.set(false);
                    error_msg.set(Some(format!("Failed to create curriculum: {}", e)));
                }
            }
        });
    };

    rsx! {
        div { class: "container",
            h1 { "Learning App" }
            p { "Enter a topic to generate a personalized curriculum." }

            div { class: "create-form",
                input {
                    r#type: "text",
                    placeholder: "What do you want to learn? (e.g., bansuri)",
                    value: "{topic}",
                    disabled: *creating.read(),
                    oninput: move |e| topic.set(e.value()),
                    onkeypress: move |e| {
                        if e.key() == Key::Enter {
                            on_submit(());
                        }
                    },
                }
                button {
                    disabled: *creating.read() || topic.read().trim().is_empty(),
                    onclick: on_submit,
                    if *creating.read() {
                        "Generating..."
                    } else {
                        "Create Curriculum"
                    }
                }
            }

            if let Some(err) = &*error_msg.read() {
                p { class: "error", "{err}" }
            }

            if !curricula.read().is_empty() {
                div { class: "curricula-list",
                    h2 { "Your Curricula" }
                    for c in curricula.read().iter() {
                        Link {
                            to: Route::CurriculumView { id: c.id.clone() },
                            class: "curriculum-card",
                            h3 { "{c.title}" }
                            span { class: "topic", "{c.topic}" }
                        }
                    }
                }
            }
        }
    }
}
