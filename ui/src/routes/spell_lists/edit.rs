use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::stores::Store;

use crate::components::edit::SpellListEdit as SpellListEditComponent;

#[component]
pub fn SpellListEdit(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.spell_lists;

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

    rsx! {
        SpellListEditComponent { list: list_locked }
    }
}
