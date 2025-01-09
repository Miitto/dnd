use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::meta::Link;
use types::spells::SpellList;
use types::stores::Store;

use crate::components::edit::StringList;
use crate::components::view::Pair;

use types::stores::Saveable;

#[derive(Props, Clone)]
pub struct SpellListEditProps {
    list: Arc<Mutex<SpellList>>,
}

impl PartialEq for SpellListEditProps {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.list.force_lock();
        let other_lock = other.list.force_lock();

        *self_lock == *other_lock
    }
}

#[component]
pub fn SpellListEdit(props: SpellListEditProps) -> Element {
    let store = use_context::<Store>();
    let all = store.spell_lists;
    let all_spells = store.spells;
    let list_locked = props.list;

    let list = list_locked.force_lock();

    // region: Signal
    let mut name = use_signal(|| list.name.clone());
    let mut spells: Signal<Vec<String>> =
        use_signal(|| list.spells.iter().map(|spell| spell.name()).collect());

    drop(list);

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
            Pair { name: "Name",
                input { value: "{name}", oninput: move |e| name.set(e.value()) }
            }
            br {}
            StringList {
                name: name(),
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
