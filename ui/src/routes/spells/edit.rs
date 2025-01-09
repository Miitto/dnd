use std::sync::{Arc, Mutex};

use dioxus::prelude::*;

use types::{extensions::ForceLock, stores::Store};

use crate::components::edit::SpellEdit as SpellEditComponent;

#[component]
pub fn SpellEdit(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.spells;
    let lists = store.spell_lists;

    let spell_locked = {
        let spell_locked = all.get(&id).unwrap_or_default();

        let spell = spell_locked.force_lock();

        if spell.name != id {
            let mut clone = (*spell).clone();
            clone.name = id.clone();
            let arc = Arc::new(Mutex::new(clone));
            all.store
                .lock()
                .expect("Failed to lock Spells when inserting new spell")
                .insert(id.clone(), arc.clone());

            let lists = lists
                .store
                .lock()
                .expect("Failed to lock SpellLists on new spell");

            lists.iter().for_each(|(_, list)| {
                (*list.force_lock()).found(arc.clone());
            });

            arc
        } else {
            drop(spell);
            spell_locked
        }
    };

    rsx! {
        SpellEditComponent { spell: spell_locked }
    }
}
