use dioxus::prelude::*;

mod weapons;
pub use weapons::*;

mod weapon;
pub use weapon::*;

use crate::routes::Routes;

#[component]
pub fn Items() -> Element {
    rsx! {
        Link { to: Routes::Weapons {}, "Weapons" }
    }
}
