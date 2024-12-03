use dioxus::prelude::*;
use dnd_types::{items::weapon::WeaponType, stores::Store};

use crate::routes::Routes;

#[component]
pub fn Weapons() -> Element {
    let mut melee_count = 0;
    let mut ranged_count = 0;

    let store = use_context::<Store>();
    let weapon_store = store.weapons;
    let lock_r = weapon_store.weapons.lock();

    if lock_r.is_err() {
        return rsx! {
            Link { to: Routes::Home {}, "Return Home"}
            "Loading Failed"
        };
    }

    let lock = lock_r.unwrap();

    lock.iter().for_each(|el| match **el {
        WeaponType::Melee(_) => {
            melee_count += 1;
        }
        WeaponType::Ranged(_) => {
            ranged_count += 1;
        }
    });

    rsx! {
        Link { to: Routes::Home {}, "Go to counter" }
        "Melee Weapons: {melee_count}"
    }
}
