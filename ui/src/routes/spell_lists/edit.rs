use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::meta::Link;
use types::stores::Store;

use crate::components::edit::StringList;

use types::stores::Saveable;

#[component]
pub fn SpellListEdit(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.spell_lists;
    let all_spells = store.spells;

    let list_locked = {
        let list_locked = all.get(&id).unwrap_or_default();

        let list = list_locked.force_lock();

        if list.name != id {
            let mut clone = (*list).clone();
            clone.name = id.clone();
            let arc = Arc::new(Mutex::new(clone));
            all.store
                .lock()
                .expect("Failed to lock lists when inserting new list")
                .insert(id.clone(), arc.clone());

            arc
        } else {
            drop(list);
            list_locked
        }
    };

    let list_lock_clone = list_locked.clone();

    let list = use_memo(move || list_lock_clone.force_lock().clone());

    // region: Signal
    let name = use_signal(move || list().name.clone());
    let mut spells: Signal<Vec<String>> =
        use_signal(move || list().spells.iter().map(|spell| spell.name()).collect());

    let _ = use_effect(move || {
        let mut list = list_locked.force_lock();

        list.name = name();

        list.spells = spells()
            .iter()
            .map(|name| {
                if let Some(spell) = all_spells.get(name) {
                    Link::Found(spell)
                } else {
                    Link::NotFound(name.clone())
                }
            })
            .collect();
    });
    // endregion

    rsx! {
        div {
            StringList {
                name: "{list().name}",
                list: spells(),
                oninput: move |list| { spells.set(list) },
            }

            br {}
            button {
                class: "px-4 py-2 rounded border w-fit h-fit",
                onclick: move |_| { all.save(name().as_str()) },
                "Save"
            }
        }
    }
}
