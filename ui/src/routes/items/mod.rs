use dioxus::prelude::*;

mod weapons;
pub use weapons::*;

mod weapon;
pub use weapon::*;

use crate::routes::Routes;

#[component]
pub fn Items() -> Element {
    rsx! {
        h1 { "Items" }
        ul { class: "list-disc pl-6",
            li {
                Link { to: Routes::Weapons {}, "Weapons" }
            }
        }
    }
}
