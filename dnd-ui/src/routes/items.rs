use dioxus::prelude::*;

mod weapons;
pub use weapons::*;

use crate::routes::Routes;

#[component]
pub fn Items() -> Element {
    rsx! {
        Link { to: Routes::Weapons {},
            "Weapons"
        }
    }
}
