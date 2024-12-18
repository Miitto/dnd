use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Home() -> Element {
    rsx! {
        Link { to: Routes::Weapons {}, "Go to weapons" }
    }
}
