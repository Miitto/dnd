use dioxus::prelude::*;
use types::{classes::CastType, stores::Store};

use crate::{
    routes::classes::class::{ClassFeature, Proficiencies},
    Ordinal,
};

#[component]
pub fn Subclass(class_id: String, subclass_id: String) -> Element {
    let store = use_context::<Store>();
    let store = store.classes;
    let class = store.get_clone(&class_id);
    let subclass = class
        .as_ref()
        .and_then(|c| c.subclasses.get(&subclass_id).cloned());

    rsx! {
        if let (Some(class), Some(subclass)) = (class.as_ref(), subclass) {
            h1 { "{subclass.name}" }
            div { class: "flex flex-col",
                for split in subclass.description.lines() {
                    p { "{split}" }
                }

                if !subclass.proficiencies.is_empty() {
                    br {}
                    h3 { "Extra Proficiencies" }
                    Proficiencies { proficiencies: subclass.proficiencies }
                }

                if !subclass.spells.is_empty() {
                    br {}
                    h3 {
                        if class.cast_type.clone().filter(|e| *e == CastType::Prepared).is_some() {
                            "Extra Spells"
                        } else {
                            "Expanded Spell List"
                        }
                    }
                    table { class: "border",
                        thead { class: "font-bold",
                            tr { class: "*:px-2 *:py-1 border-b *:text-left",
                                th { "Level" }
                                th { "Spells" }
                            }
                        }
                        tbody {
                            for lvl in (1..=20) {
                                if let Some(spells) = subclass.spells.get(&lvl) {
                                    tr { class: "*:px-2 *:py-1 even:bg-muted",
                                        td { "{lvl.ordinal()}" }
                                        td { "{spells.join(\", \")}" }
                                    }
                                }
                            }
                        }
                    }
                }

                for lvl in (1..=20) {
                    if let Some(features) = subclass.features.get(&lvl) {
                        hr { class: "my-4" }
                        h2 { class: "font-bold", "{lvl.ordinal()} level" }
                        for feature in features.iter() {
                            ClassFeature { feature: feature.clone() }
                        }
                    }
                }
            }
        } else {
            if class.is_some() {
                "Subclass not found"
            } else {
                "Class not found"
            }
        }
    }
}
