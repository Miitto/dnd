use std::sync::{Arc, Mutex};

use dioxus::logger::tracing;
use dioxus::prelude::*;
use types::background::Background;
use types::extensions::ForceLock;
use types::meta::{Description, NamedDescription};
use types::stores::Store;
use types::traits::Linkable;

use crate::components::edit::{
    DescriptionInputSignal, NameDescriptionListSignal, SkillMultiSelect, SourceInputSignal,
    StringListSignal,
};
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
    let all = store.backgrounds;
    let background_locked = props.background;

    let background = background_locked.force_lock();

    // region: Signal
    let mut name = use_signal(|| background.name.clone());
    let mut category = use_signal(|| background.category.clone());
    let source = use_signal(|| background.source.clone());
    let description = use_signal(|| background.description.clone());
    let equipment: Signal<Vec<String>> =
        use_signal(|| background.equipment.iter().map(|e| e.to_string()).collect());
    let tool_proficiencies = use_signal(|| background.tool_proficiencies.clone());
    let mut languages = use_signal(|| background.languages.clone());
    let skill_list = use_signal(|| background.skill_proficiencies.clone());
    let features = use_signal(|| {
        background
            .features
            .iter()
            .map(|f| (f.name.clone(), f.description.clone()))
            .collect::<Vec<(String, Description)>>()
    });

    drop(background);

    let _ = use_effect(move || {
        let mut background = background_locked.force_lock();

        background.name = name();
        background.source = source();
        background.category = category();
        background.description = description();
        background.equipment = equipment().into_iter().map(Description::from).collect();
        background.languages = languages();
        background.skill_proficiencies = skill_list();
        background.tool_proficiencies = tool_proficiencies();
        background.features = features()
            .iter()
            .filter_map(|(k, v)| {
                if k.is_empty() {
                    None
                } else {
                    Some(NamedDescription {
                        name: k.clone(),
                        description: v.clone(),
                        ..Default::default()
                    })
                }
            })
            .collect();

        background.link();
    });
    // endregion

    rsx! {
        div { class: "flex flex-col gap-y-2",
            div { class: "grid grid-cols-auto-fr gap-y-2",
                Pair { name: "Name", align: true, grid: true,
                    input {
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                }
                Pair { name: "Category", align: true, grid: true,
                    input {
                        value: "{category}",
                        oninput: move |e| category.set(e.value()),
                    }
                }
                Pair { name: "Source", align: true, grid: true,
                    SourceInputSignal { source }
                }
            }
            br {}

            h2 { "Description" }
            DescriptionInputSignal { description }

            Pair { name: "Languages", align: true, class: "flex",
                input {
                    class: "flex-grow",
                    value: "{languages}",
                    oninput: move |e| languages.set(e.value()),
                }
            }

            StringListSignal { name: "Equipment", list: equipment }
            br {}
            StringListSignal { name: "Tool Proficiencies", list: tool_proficiencies }

            div {
                h2 { "Skill Proficiencies" }
                SkillMultiSelect { list: skill_list }
            }

            h2 { "Features" }
            NameDescriptionListSignal { list: features }

            br {}
            button {
                class: "px-4 py-2 rounded border w-fit h-fit",
                onclick: move |_| {
                    all.save(name().as_str()).unwrap_or_else(|e| tracing::error!("{}", e));
                },
                "Save"
            }
        }
    }
}
