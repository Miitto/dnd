use std::sync::{Arc, Mutex};

use dioxus::logger::tracing;
use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::race::Race;
use types::stores::Store;

use crate::components::edit::{
    AttributesInputSignal, DescriptionInputSignal, MultiTableEdit, SizeSelectorSignal,
    SourceInputSignal,
};
use crate::components::view::Pair;

use types::stores::Saveable;

#[derive(Props, Clone)]
pub struct RaceEditProps {
    race: Arc<Mutex<Race>>,
}

impl PartialEq for RaceEditProps {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.race.force_lock();
        let other_lock = other.race.force_lock();

        *self_lock == *other_lock
    }
}

#[component]
pub fn RaceEdit(props: RaceEditProps) -> Element {
    let store = use_context::<Store>();
    let all = store.races;
    let race_locked = props.race;

    let race = race_locked.force_lock();

    // region: Signal
    let mut name = use_signal(|| race.name.clone());
    let source = use_signal(|| race.source.clone());
    let description = use_signal(|| race.description.clone());
    let default_asi = use_signal(|| race.default_asi.clone());
    let mut age = use_signal(|| race.age.clone());
    let mut alignment = use_signal(|| race.alignment.clone());
    let size = use_signal(|| race.size.size);
    let mut size_description = use_signal(|| race.size.description.clone());
    let mut speed = use_signal(|| race.speed);
    let mut languages = use_signal(|| race.languages.clone());
    let mut tables = use_signal(|| race.tables.clone());
    let mut category = use_signal(|| race.category.clone());
    let features = use_signal(|| race.features.clone());

    drop(race);
    //endregion

    // region: Effects
    let _ = use_effect(move || {
        let mut race = race_locked.force_lock();

        race.name = name();
        race.category = category();
        race.source = source();
        race.description = description();
        race.default_asi = default_asi();
        race.age = age();
        race.alignment = alignment();
        race.size.size = size();
        race.size.description = size_description();
        race.speed = speed();
        race.languages = languages();
        race.tables = tables();
        race.features = features();
    });
    // endregion

    rsx! {
        div { class: "flex flex-col w-full gap-y-2",
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

            h2 { "Attributes" }
            AttributesInputSignal { attributes: default_asi }
            br {}

            Pair { name: "Age", align: true,
                input {
                    class: "w-full",
                    value: "{age}",
                    oninput: move |e| age.set(e.value()),
                }
            }

            Pair { name: "Alignment", align: true,
                input {
                    class: "w-full",
                    value: "{alignment}",
                    oninput: move |e| alignment.set(e.value()),
                }
            }

            Pair { name: "Size", align: true,
                span { class: "flex flex-row flex-auto gap-2",
                    SizeSelectorSignal { size }
                    input {
                        class: "w-full",
                        value: "{size_description}",
                        oninput: move |e| size_description.set(e.value().into()),
                    }
                }
            }

            Pair { name: "Speed", align: true,
                input {
                    r#type: "number",
                    value: "{speed}",
                    oninput: move |e| speed.set(e.value().parse().unwrap_or_default()),
                }
            }

            Pair { name: "Languages", align: true,
                input {
                    class: "w-full",
                    value: "{languages}",
                    oninput: move |e| languages.set(e.value()),
                }
            }

            h2 { "Tables" }

            MultiTableEdit { tables: tables(), onchange: move |t| { tables.set(t) } }

            Pair { name: "Unique", align: true,
                span { "TODO: Implement unique" }
            }

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
