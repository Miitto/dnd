use dioxus::prelude::*;
use types::{items::Item, stores::Store};

use crate::routes::Routes;

#[component]
pub fn Weapons() -> Element {
    let mut melee_count = 0;
    let mut ranged_count = 0;

    let store = use_context::<Store>();
    let weapon_store = store.weapons;
    {
        let lock_r = weapon_store.weapons.lock();

        if lock_r.is_err() {
            return rsx! {
                Link { to: Routes::Home {}, "Return Home" }
                "Loading Failed"
            };
        }

        let lock = lock_r.unwrap();

        lock.iter().for_each(|el| {
            if el.is_melee() {
                melee_count += 1;
            } else {
                ranged_count += 1;
            }
        });
    }

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
                li {
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
