#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod layouts;
mod nav;

mod routes;
use dnd_types::stores::Store;
use routes::Routes;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new().with_custom_head(
        r#"<link rel="stylesheet" href="./assets/tailwind.css">
        <link rel="stylesheet" href="tailwind.css">"#
            .to_string(),
    );
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let store = Store::from_path("assets/resources").unwrap();
    dbg!(&store);

    use_context_provider(|| store);

    rsx! {
        Router::<Routes> {}
    }
}

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        "Page Not Found",
        Link { to: Routes::Home {}, "Return Home" }
    }
}
