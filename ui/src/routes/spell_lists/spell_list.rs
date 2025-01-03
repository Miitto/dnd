use std::{collections::HashMap, sync::Arc};

use crate::{components::ReplaceLink, Ordinal};
use dioxus::{logger::tracing, prelude::*};
use types::{spells::Spell, stores::Store};

use crate::routes::Routes;

#[component]
pub fn SpellList(id: String, page: u8) -> Element {
    let store = use_context::<Store>();
    let store = store.spell_lists;

    let list = {
        let id = id.clone();
        use_signal(move || store.get(&id))
    };

    #[derive(Debug, Clone, PartialEq)]
    struct Found {
        found: Vec<Arc<Spell>>,
        unfound: Vec<String>,
    }

    let partition = use_memo(move || {
        tracing::debug!("Partitioning spell list");
        if let Some(list) = list() {
            let (found, unfound): (Vec<_>, Vec<_>) =
                list.spells.iter().cloned().partition(|spell| match spell {
                    types::spells::SpellEntry::Name(_) => false,
                    types::spells::SpellEntry::Spell(_) => true,
                });

            let found: Vec<Arc<Spell>> = found
                .into_iter()
                .map(|spell| match spell {
                    types::spells::SpellEntry::Name(_) => unreachable!(),
                    types::spells::SpellEntry::Spell(spell) => spell,
                })
                .collect();

            let unfound: Vec<String> = unfound
                .into_iter()
                .map(|spell| match spell {
                    types::spells::SpellEntry::Name(name) => name,
                    types::spells::SpellEntry::Spell(_) => unreachable!(),
                })
                .collect();

            Some(Found { found, unfound })
        } else {
            None
        }
    });

    let levelled = use_memo(move || {
        tracing::debug!("Leveling spells");
        if let Some(Found { found: list, .. }) = partition() {
            let mut spells = HashMap::new();

            for spell in list.iter() {
                let level = spell.level;
                let spell = Arc::clone(spell);

                let entry = spells.entry(level).or_insert_with(Vec::new);
                entry.push(spell);
            }

            Some(spells)
        } else {
            None
        }
    });

    let levels = use_memo(move || {
        if let Some(levelled) = levelled() {
            let mut levels = levelled.keys().copied().collect::<Vec<_>>();
            levels.sort_unstable();

            levels
        } else {
            vec![]
        }
    });

    let name = list().map(|list| list.name.clone()).unwrap_or_default();

    rsx! {
        if let (Some(Found { unfound, .. }), Some(levelled)) = (partition(), levelled()) {
            h1 { class: "underline", "{name} Spell List" }
            br {}
            div { class: "flex flex-wrap flex-row gap-1",
                for level in levels() {
                    LevelButton { id: id.clone(), page, level }
                }
                if let Some(Found { unfound, .. }) = partition() {
                    if !unfound.is_empty() {
                        LevelButton { id: id.clone(), page, level: u8::MAX }
                    }
                }
            }
            div { class: "p-4 pb-6 border rounded-b-md",
                table { class: "w-full",
                    tr { class: "*:text-left *:p-2 border-b",
                        if page == u8::MAX {
                            th { "Name" }
                        } else {
                            th { "Name" }
                            th { "School" }
                            th { "Casting Time" }
                            th { "Range" }
                            th { "Duration" }
                            th { "Components" }
                        }
                    }
                    if page == u8::MAX {
                        for spell in unfound {
                            tr { class: "*:p-2 border-b",
                                td {
                                    Link { to: Routes::Spell { id: spell.clone() }, "{spell}" }
                                }
                            }
                        }
                    } else {
                        for spell in levelled.get(&page).unwrap_or(&Vec::new()) {
                            tr { class: "*:p-2 border-b",
                                td {
                                    Link {
                                        to: Routes::Spell {
                                            id: spell.name.clone(),
                                        },
                                        "{spell.name}"
                                    }
                                }
                                td { class: "italic", "{spell.school}" }
                                td { "{spell.cast_time}" }
                                td { "{spell.range} feet" }
                                td { "{spell.duration}" }
                                td { "{spell.components}" }
                            }
                        }
                    }
                }
            }
        } else {
            "Can't find spell list"
        }
    }
}

#[component]
fn LevelButton(id: String, page: u8, level: u8) -> Element {
    rsx! {
        ReplaceLink {
            class: format!(
                "px-4 py-2 rounded-t-md [&.active]:bg-primary [&.active]:text-primary-foreground border border-b-0 {}",
                if page == level { "active" } else { "" },
            ),
            to: Routes::SpellList {
                id: id.clone(),
                page: level,
            },
            if level == 0 {
                "Cantrips"
            } else if level == u8::MAX {
                "?"
            } else {
                "{level.ordinal()}"
            }
        }
    }
}
