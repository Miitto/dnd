use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Nav() -> Element {
    rsx! {
        nav { class: "flex w-fit p-4 py-2", Navbar {} }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        ul { class: "flex gap-4 flex-col md:flex-row",
            li {
                Link { to: Routes::Home {}, "Home" }
            }
            li {
                Link { to: Routes::Items {}, "Items" }
            }
        }
    }
}
