use std::sync::Arc;

use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;
use crate::Ordinal;

use crate::components::info::Pair;

use types::spells::{Spell as SpellT, SpellList};

#[component]
pub fn Spell(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.spells;
    let lists = store.spell_lists;

    let arc_id = Arc::new(id);
    let cloned_id = arc_id.clone();
    let list_id_clone = arc_id.clone();

    let spell = use_memo(move || all.get(&cloned_id));

    let spell_lists = use_memo(move || {
        let lock = lists.store.lock();
        if let Ok(lists) = lock {
            lists
                .iter()
                .filter(|list| {
                    list.spells
                        .iter()
                        .any(|spell| spell.name() == list_id_clone.as_str())
                })
                .cloned()
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    rsx! {
        if let Some(spell) = spell() {
            SpellView { spell, spell_lists: spell_lists() }
        } else {
            h1 { "Spell Not Found" }
            Link {
                to: Routes::SpellEdit {
                    id: (*arc_id).clone(),
                },
                "Edit Spell"
            }
        }
    }
}

#[component]
fn SpellView(spell: Arc<SpellT>, spell_lists: Vec<Arc<SpellList>>) -> Element {
    rsx! {
        span { class: "w-full inline-flex justify-between items-center",
            h1 { "{spell.name}" }
            Link {
                to: Routes::SpellEdit {
                    id: spell.name.to_string(),
                },
                "Edit"
            }
        }

        div { class: "flex flex-col",

            p { class: "italic",
                if spell.level == 0 {
                    "{spell.school} cantrip"
                } else {
                    "{spell.level.ordinal()}-Level {spell.school}"
                }
            }
            br {}
            Pair { name: "Casting Time", "{spell.cast_time}" }
            Pair { name: "Range", "{spell.range} feet" }
            Pair { name: "Components", "{spell.components}" }
            Pair { name: "Duration", "{spell.duration}" }

            if spell.concentration {
                p { class: "italic", "Requires Concentration" }
            }
            if spell.ritual {
                p { class: "italic", "Ritual" }
            }

            if let Some(save) = spell.save.as_ref() {
                br {}
                Pair { name: "Saving Throw", "{save}" }
                if let Some(on_save) = spell.on_save.as_ref() {
                    Pair { name: "On Save", "{on_save}" }
                }
            }
            br {}

            p { "{spell.description}" }

            if let Some(higher) = spell.at_higher_levels.as_ref() {
                br {}
                Pair { name: "At Higher Levels", "{higher}" }
            }
            br {}

            Pair { name: "Spell Lists",
                for (idx , list) in spell_lists.iter().enumerate() {
                    Link {
                        to: Routes::SpellList {
                            id: list.name.clone(),
                            page: spell.level,
                        },
                        "{spell.name}"
                    }
                    if idx < spell_lists.len() - 1 {
                        ", "
                    }
                }
            }
        }
    }
}
