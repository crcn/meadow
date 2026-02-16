mod components;
mod routes;
mod server_fns;

use dioxus::prelude::*;
use routes::Route;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
