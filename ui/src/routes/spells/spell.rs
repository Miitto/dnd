use std::sync::Arc;

use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::stores::Store;

use crate::components::view::spell::SpellView;
use crate::routes::Routes;

#[component]
pub fn Spell(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.spells;
    let lists = store.spell_lists;

    let arc_id = Arc::new(id);
    let cloned_id = arc_id.clone();
    let list_id_clone = arc_id.clone();

    let spell = all.get_clone(&cloned_id);

    let spell_lists = use_memo(move || {
        let lock = lists.store.lock();
        if let Ok(lists) = lock {
            lists
                .iter()
                .filter_map(|(_, list)| {
                    let lock = list.force_lock();
                    if lock
                        .spells
                        .iter()
                        .any(|spell| spell.name() == list_id_clone.as_str())
                    {
                        Some(lock.name.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
        } else {
            vec![]
        }
    });

    rsx! {
        if let Some(spell) = spell {
            span { class: "w-full inline-flex justify-between items-center",
                h1 { "{spell.name}" }
                if cfg!(debug_assertions) {
                    Link {
                        to: Routes::SpellEdit {
                            id: spell.name.to_owned(),
                        },
                        "Edit"
                    }
                }
            }
            SpellView { spell, spell_lists: spell_lists() }
        } else {
            h1 { "Spell Not Found" }
            if cfg!(debug_assertions) {
                Link {
                    to: Routes::SpellEdit {
                        id: (*arc_id).clone(),
                    },
                    "Edit Spell"
                }
            }
        }
    }
}
