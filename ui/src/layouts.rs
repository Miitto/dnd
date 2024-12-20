use dioxus::prelude::*;
use types::stores::Store;

use crate::{
    nav::{Breadcrumbs, Nav},
    routes::Routes,
};

#[derive(Clone, Copy)]
struct DarkMode(bool);

#[component]
pub fn NavLayout() -> Element {
    let dark_mode = use_signal(|| DarkMode(true));

    use_context_provider(|| dark_mode);

    let text = if dark_mode().0 { "dark" } else { "" };

    let mut store = use_context::<Store>();

    let mut result = use_signal(|| false);

    rsx! {
        div { class: "{text} flex-grow bg-background text-foreground flex flex-col md:flex-row flex-auto flex-shrink-0 h-full",
            Nav {}
            div { class: "flex flex-col flex-grow",
                div { class: "flex flex-row",
                    Breadcrumbs {}
                    button {
                        onclick: move |_| {
                            let mut inner = result.write();
                            let result = store.rebuild();
                            *inner = result.is_err();
                        },
                        class: "px-4",
                        "Rebuild"
                    }
                }
                main { class: "p-4 py-2", Outlet::<Routes> {} }
            }
        }
        div {
            if result() {
                div { class: "bg-red-500 text-white p-4 fixed bottom-1 right-1",
                    "Failed to rebuild store"
                }
            }
        }
    }
}
