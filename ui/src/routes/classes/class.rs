use std::sync::Arc;

use dioxus::prelude::*;
use types::{
    classes::{Class as ClassT, ClassFeature},
    is_asi_level, proficiency_bonus,
    stores::Store,
};

use crate::{components::info::Pair, DashIfZero, Ordinal};

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
                ClassTable { class: class.clone() }
                h2 { "Class Features" }
                br {}
                h3 { "Hit Points" }
                Pair { name: "Hit Dice", "1d{class.hit_die}" }
                Pair { name: "Hit Points at 1st Level", "{class.hit_die} + Con mod" }
                Pair { name: "Hit Points at Higher Levels",
                    "1d{class.hit_die} (or {class.hit_die / 2 + 1})+ Con mod per level after 1st"
                }
                br {}
                h3 { "Proficiencies" }
                Pair { name: "Armor", "{class.proficiencies.armor.join(\", \")}" }
                Pair { name: "Weapons", "{class.proficiencies.weapons.join(\", \")}" }
                Pair { name: "Tools", "{class.proficiencies.tools.join(\", \")}" }
                Pair { name: "Saving Throws", "{class.proficiencies.saving_throws.join(\", \")}" }
                Pair { name: "Skills", "{class.proficiencies.skills}" }

                br {}
                h3 { "Equipment" }

                ul { class: "list-disc pl-6",
                    for item in class.equipment.iter() {
                        li { "{item}" }
                    }
                }

                if let Some(spell_mod) = class.spellcasting {
                    br {}
                    h3 { "Spellcasting Ability" }
                    p { "{spell_mod}" }
                    p {
                        b { "Spell Save DC:" }
                        " 8 + your proficiency bonus + your {spell_mod} modifier"
                    }
                    p {
                        b { "Spell Attack Modifier:" }
                        " your proficiency bonus + your {spell_mod} modifier"
                    }
                }

                if class.ritual_casting {
                    br {}
                    h3 { "Ritual Casting" }
                    p {
                        "You can cast an artificer spell as a ritual if that spell has the ritual tag and you have the spell prepared."
                    }
                }

                if let Some(features) = class.features.get(&0) {
                    for ClassFeature { name , description } in features.iter() {
                        br {}
                        h3 { "{name}" }
                        p { "{description}" }
                    }
                }

                for lvl in (1..=20) {
                    if let Some(features) = class.features.get(&lvl) {
                        hr { class: "my-4" }
                        h2 { class: "font-bold", "{lvl.ordinal()} level" }
                        for ClassFeature { name , description } in features.iter() {
                            br {}
                            h3 { "{name}" }
                            p { "{description}" }
                        }
                    }
                }
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
                                    for ClassFeature { name , .. } in feature.iter() {
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
