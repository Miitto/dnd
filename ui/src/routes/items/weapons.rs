use dioxus::prelude::*;
use types::{items::Item, stores::Store};

use crate::routes::Routes;

#[component]
pub fn Weapons() -> Element {
    let store = use_context::<Store>();
    let weapon_store = store.weapons;

    let sorted_melee = use_hook(|| {
        let mut melee = weapon_store.melee();

        melee.sort_by_key(|w| w.name().to_string());

        melee
    });

    rsx! {
        h1 { class: "underline", "Weapons" }
        h2 { "Melee" }
        ul { class: "list-disc pl-6",
            for melee in sorted_melee {
                li { key: melee.name(),
                    Link {
                        to: Routes::Weapon {
                            id: melee.name().to_string(),
                        },
                        "{melee.name()}"
                    }
                }
            }
        }
    }
}
