use dioxus::prelude::*;

use types::aliased::*;

use crate::{
    components::view::{Description, Pair, StatBlockView},
    routes::Routes,
    Ordinal,
};

use types::extensions::ForceLock;

#[component]
pub fn SpellView(spell: SpellT, spell_lists: Vec<String>) -> Element {
    let range_unit = if spell.range.parse::<i32>().is_ok() {
        " feet"
    } else {
        ""
    };

    let concentration_str = if spell.concentration {
        "Concentration, "
    } else {
        ""
    };

    let ritual_str = if spell.ritual { " (Ritual)" } else { "" };

    rsx! {
        span { class: "w-full inline-flex justify-between items-center",
            h1 { "{spell.name}" }
            Link {
                to: Routes::SpellEdit {
                    id: spell.name.to_owned(),
                },
                "Edit"
            }
        }

        div { class: "flex flex-col",

            p { class: "italic",
                if spell.level == 0 {
                    "{spell.school} cantrip"
                } else {
                    "{spell.level.ordinal()}-Level {spell.school} {ritual_str}"
                }
            }
            br {}
            Pair { name: "Casting Time", "{spell.cast_time}" }
            Pair { name: "Range", "{spell.range}{range_unit}" }
            Pair { name: "Components", "{spell.components}" }
            Pair { name: "Duration", "{concentration_str}{spell.duration}" }

            if spell.concentration {
                p { class: "italic", "Requires Concentration" }
            }
            if spell.ritual {
                p { class: "italic", "Ritual" }
            }

            if let Some(save) = spell.save.as_ref() {
                br {}
                Pair { name: "Saving Throw", "{save}" }
                Pair { name: "On Save", "{spell.on_save.unwrap_or_default()}" }
            }
            br {}

            Description { description: spell.description }

            if let Some(higher) = spell.at_higher_levels.as_ref() {
                br {}
                Pair { name: "At Higher Levels", "{higher}" }
            }
            br {}

            Pair { name: "Spell Lists",
                for (idx , list) in spell_lists.iter().enumerate() {
                    Link {
                        to: Routes::SpellList {
                            id: list.clone(),
                            page: spell.level,
                        },
                        "{list}"
                    }
                    if idx < spell_lists.len() - 1 {
                        ", "
                    }
                }
            }
            if !spell.appended_stat_blocks.is_empty() {
                br {}
                for stat_block in spell.appended_stat_blocks.iter() {
                    if let types::meta::Link::Found(stat_block) = stat_block {
                        StatBlockView { stat_block: stat_block.force_lock().clone() }
                    }
                }
            }
        }
    }
}
