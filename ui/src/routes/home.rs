use dioxus::prelude::*;
use types::{extensions::ForceLock, fs::classes::class, stores::Store};

use crate::routes::Routes;
use types::stores::Saveable;

#[component]
pub fn Home() -> Element {
    let store = use_context::<Store>();

    let races = store.races;
    let backgrounds = store.backgrounds;
    let classes = store.classes;
    let subclasses = classes.clone();
    let feats = store.feats;
    let spell_lists = store.spell_lists;

    let race = races.store.force_lock().keys().next().unwrap().clone();
    let background = backgrounds
        .store
        .force_lock()
        .keys()
        .next()
        .unwrap()
        .clone();
    let class = classes.store.force_lock().keys().next().unwrap().clone();
    let subclass = (
        class.clone(),
        classes
            .store
            .force_lock()
            .get(class.as_str())
            .unwrap()
            .force_lock()
            .subclasses
            .options
            .keys()
            .next()
            .unwrap()
            .clone(),
    );
    let feat = feats.store.force_lock().keys().next().unwrap().clone();
    let spell_list = spell_lists
        .store
        .force_lock()
        .keys()
        .next()
        .unwrap()
        .clone();

    rsx! {
        div { class: "flex flex-col w-fit",
            h1 { "Save tests" }
            div { class: "flex flex-row flex-wrap gap-4",
                button {
                    onclick: move |_| {
                        races.save(race.as_str());
                    },
                    "Race"
                }

                button {
                    onclick: move |_| {
                        backgrounds.save(background.as_str());
                    },
                    "Background"
                }

                button {
                    onclick: move |_| {
                        classes.save(class.as_str());
                    },
                    "Class"
                }

                button {
                    onclick: move |_| {
                        subclasses.save(format!("{}/{}", subclass.0, subclass.1).as_str());
                    },
                    "Subclass"
                }

                button {
                    onclick: move |_| {
                        feats.save(feat.as_str());
                    },
                    "Feat"
                }

                button {
                    onclick: move |_| {
                        spell_lists.save(spell_list.as_str());
                    },
                    "Spell List"
                }
            }
        }
    }
}
