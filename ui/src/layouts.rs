use dioxus::prelude::*;

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

    rsx! {
        div { class: "{text} h-full bg-background text-foreground flex flex-col md:flex-row",
            Nav {}
            div { class: "flex flex-col",
                Breadcrumbs {}
                main { class: "p-4 py-2", Outlet::<Routes> {} }
            }
        }
    }
}
