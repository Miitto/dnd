use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use types::background::Background;
use types::extensions::ForceLock;
use types::stores::Store;

use crate::components::edit::{SkillMultiSelect, StringList};
use crate::components::view::Pair;

use types::stores::Saveable;

#[derive(Props, Clone)]
pub struct BackgroundEditProps {
    background: Arc<Mutex<Background>>,
}

impl PartialEq for BackgroundEditProps {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.background.force_lock();
        let other_lock = other.background.force_lock();

        *self_lock == *other_lock
    }
}

#[component]
pub fn BackgroundEdit(props: BackgroundEditProps) -> Element {
    let store = use_context::<Store>();
    let all = store.spell_lists;
    let background_locked = props.background;

    let background = background_locked.force_lock();

    // region: Signal
    let mut name = use_signal(|| background.name.clone());
    let mut description = use_signal(|| background.description.to_string());
    let mut equipment = use_signal(|| background.equipment.clone());
    let mut languages = use_signal(|| background.languages.clone());
    let skill_list = use_signal(|| background.skill_proficiencies.clone());

    drop(background);

    let _ = use_effect(move || {
        let mut background = background_locked.force_lock();

        background.name = name();
        background.description = description().into();
    });
    // endregion

    rsx! {
        div {
            Pair { name: "Name", align: true,
                input { value: "{name}", oninput: move |e| name.set(e.value()) }
            }
            br {}

            h2 { "Description" }
            textarea {
                class: "w-full",
                value: "{description}",
                oninput: move |e| description.set(e.value()),
            }

            Pair { name: "Languages", align: true,
                input {
                    value: "{languages}",
                    oninput: move |e| languages.set(e.value()),
                }
            }


            StringList {
                name: "Equipment",
                list: equipment(),
                oninput: move |list| equipment.set(list),
            }

            div {
                h2 { "Skill Proficiencies" }
                SkillMultiSelect { list: skill_list }
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
