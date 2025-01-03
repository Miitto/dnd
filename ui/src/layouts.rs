use dioxus::prelude::*;

use crate::{
    components::nav::{Breadcrumbs, Nav},
    routes::Routes,
};

#[derive(Clone, Copy)]
struct DarkMode(bool);

#[component]
pub fn NavLayout() -> Element {
    let dark_mode = use_signal(|| DarkMode(true));

    use_context_provider(|| dark_mode);

    let text = if dark_mode().0 { "dark" } else { "" };

    let mut pin_breadcrumbs = use_signal(|| true);

    let sticky = if pin_breadcrumbs() {
        "sticky top-0"
    } else {
        ""
    };

    let pin_txt = if pin_breadcrumbs() { "Unpin" } else { "Pin" };

    rsx! {
        div { class: "{text} flex-grow bg-background text-foreground flex flex-col md:flex-row flex-auto flex-shrink-0 h-full",
            Nav {}
            div { class: "flex flex-col flex-grow",
                div { class: "flex flex-row bg-background border-b {sticky}",
                    Breadcrumbs {}
                    button {
                        onclick: move |_| {
                            let mut v = pin_breadcrumbs.write();
                            *v = !*v;
                        },
                        class: "px-4",
                        "{pin_txt}"
                    }
                }
                main { class: "px-4 pt-2 pb-8", Outlet::<Routes> {} }
            }
        }
    }
}
