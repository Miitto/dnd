#![allow(non_snake_case)]

use dioxus::{logger::tracing, prelude::*};

mod layouts;
mod nav;
mod routes;

const FAVICON: Asset = asset!("../assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("../assets/styling/tailwind.css");

const RESOURCES: Asset = asset!("../assets/resources");

use routes::Routes;
use types::stores::Store;

#[component]
pub fn App() -> Element {
    let bundled = RESOURCES.bundled();

    let abs = bundled.absolute_source_path();
    tracing::debug!("Resource Dir: {:?}", abs);

    let store = Store::from_path(abs).unwrap();
    use_context_provider(|| store);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }

        Router::<Routes> {}
    }
}

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        "Page Not Found"
        Link { to: Routes::Home {}, "Return Home" }
    }
}
