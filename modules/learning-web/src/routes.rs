use dioxus::prelude::*;

use crate::components::{curriculum_view::CurriculumView, home::Home, node_view::NodeView};

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/curriculum/:id")]
    CurriculumView { id: String },

    #[route("/node/:id")]
    NodeView { id: String },
}
