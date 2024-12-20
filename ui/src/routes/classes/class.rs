use std::sync::Arc;

use dioxus::prelude::*;
use types::{is_asi_level, stores::Store};

use crate::{DashIfZero, Ordinal};
use types::{classes::class::Class as ClassT, proficiency_bonus};

#[component]
pub fn Class(id: String) -> Element {
    let store = use_context::<Store>();
    let class_store = store.classes;
    let class = use_memo(move || class_store.get(&id));

    rsx! {
        if let Some(class) = class() {
            h1 { "{class.name}" }

            div { class: "flex flex-col",
                p { "{class.description}" }
                p { class: "py-1 italic",
                    "You must have a{class.requirements_string_prepend()} to multi-class in or out of this class"
                }
                ClassTable { class }
            }
        } else {
            "class not found"
        }
    }
}

#[component]
pub fn ClassTable(class: Arc<ClassT>) -> Element {
    rsx! {
        table { class: "border",
            thead {
                tr { class: "*:px-2 *:py-1 *:text-left border-b",
                    th { "Level" }
                    th { "Proficiency Bonus" }
                    th { "Features" }
                    for (name , _) in class.table_entries.iter() {
                        th { "{name}" }
                    }
                    th { "Cantrips Known" }
                    for level in (1..=class.cast_level.max_level()) {
                        th { "{level.ordinal()}" }
                    }
                }
            }
            tbody {
                for level in (1..=20) {
                    tr { class: "*:p-2 border-b",
                        td { "{level.ordinal()}" }
                        td { "+{proficiency_bonus(level)}" }
                        td {
                            ul { class: "list-disc list-inside",
                                if is_asi_level(level) {
                                    li { "ASI" }
                                }
                                for (_ , feature) in class.features.iter().filter(|(&lvl, _)| lvl == level) {
                                    for (name , _) in feature.iter() {
                                        li { "{name}" }
                                    }
                                }
                            }
                        }
                        for (_ , entry) in class.table_entries.iter() {
                            td { "{entry.get(level)}" }
                        }
                        td { "{class.cantrips_known(level)}" }
                        for spell_level in (1..=class.cast_level.max_level()) {
                            td { "{class.spell_slots(level, spell_level).dash_if_zero()}" }
                        }
                    }
                }
            }
        }
    }
}
