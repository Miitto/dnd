#![allow(non_snake_case)]

use dioxus::{logger::tracing, prelude::*};

pub mod components;
mod layouts;
mod nav;
mod routes;

const FAVICON: Asset = asset!("assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("assets/styling/tailwind.css");

const RESOURCES: Asset = asset!("assets/resources");

use routes::Routes;
use types::stores::Store;

#[component]
pub fn App() -> Element {
    let bundled = RESOURCES.bundled();

    let abs = bundled.absolute_source_path();
    tracing::debug!("Resource Dir: {:?}", abs);

    let store = Store::from_path(abs);
    use_context_provider(|| store);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }

        Router::<Routes> {
            config: || {
                RouterConfig::default()
                    .on_update(|_| {
                        document::eval("window.scrollTo(0, 0)");
                        None
                    })
            },
        }
    }
}

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        "Page Not Found"
        Link { to: Routes::Home {}, "Return Home" }
    }
}

pub trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for str {
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        chars
            .next()
            .map(|c| c.to_uppercase())
            .into_iter()
            .flatten()
            .chain(chars)
            .collect()
    }
}

impl Capitalize for String {
    fn capitalize(&self) -> String {
        self.as_str().capitalize()
    }
}

pub trait Ordinal {
    fn ordinal(&self) -> String;
}

impl Ordinal for u8 {
    fn ordinal(&self) -> String {
        let suffix = match self % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        format!("{}{}", self, suffix)
    }
}

pub trait DashIfZero {
    fn dash_if_zero(&self) -> String;
}

impl DashIfZero for u8 {
    fn dash_if_zero(&self) -> String {
        if *self == 0 {
            "-".to_string()
        } else {
            self.to_string()
        }
    }
}
