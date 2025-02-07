use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use types::aliased::SpellListT;

use crate::routes::Routes;

use crate::components::ReplaceLink;
use crate::Ordinal;

pub fn make_dyn_level_button(id: String, page: u8) -> Callback<u8, Element> {
    Callback::new(move |level| {
        rsx! {
            LevelButton { id: id.clone(), page, level }
        }
    })
}

#[component]
pub fn SpellListView(
    list: Arc<SpellListT>,
    page: u8,
    level_button: Callback<u8, Element>,
) -> Element {
    let list_clone = Arc::clone(&list);

    let partition = list_clone.partitioned();

    let levelled = {
        let list = partition.0;
        let mut spells = HashMap::new();

        for spell in list {
            let spell = spell.lock().unwrap();
            let level = spell.level;

            let entry = spells.entry(level).or_insert_with(Vec::new);
            entry.push(spell.clone());
        }

        spells
    };

    let levels = {
        let mut levels = levelled.keys().copied().collect::<Vec<_>>();
        levels.sort_unstable();

        levels
    };

    rsx! {
        if list.spells.is_empty() {
            "No spells in this list"
        } else {
            div { class: "flex flex-wrap flex-row gap-1",
                for level in levels {
                    {level_button(level)}
                }
                if !partition.1.is_empty() {
                    {level_button(u8::MAX)}
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
                        for spell in partition.1 {
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
                                td { class: "italic",
                                    "{spell.school}"
                                    if spell.ritual {
                                        " (R)"
                                    } else {
                                        ""
                                    }
                                }
                                td {
                                    {
                                        let mut split = spell.cast_time.split(", ");
                                        let first = split.next().unwrap();
                                        let rest = split.next().map(|_| "...".to_string()).unwrap_or_default();
                                        format!("{}{}", first, rest)
                                    }
                                }
                                td {
                                    {
                                        format!(
                                            "{}{}",
                                            spell.range,
                                            if spell.range.parse::<i32>().is_ok() { " feet" } else { "" },
                                        )
                                    }
                                }
                                td { "{spell.duration}" }
                                td { "{spell.components}" }
                            }
                        }
                    }
                }
            }
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
