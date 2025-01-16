use std::sync::{Arc, Mutex};

use dioxus::logger::tracing;
use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::feat::Feat;
use types::stores::Store;

use crate::components::edit::{
    AttributesInputSignal, DescriptionInputSignal, SourceInputSignal, StringListSignal,
};
use crate::components::view::Pair;

use types::stores::Saveable;

#[derive(Props, Clone)]
pub struct FeatEditProps {
    feat: Arc<Mutex<Feat>>,
}

impl PartialEq for FeatEditProps {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.feat.force_lock();
        let other_lock = other.feat.force_lock();

        *self_lock == *other_lock
    }
}

#[component]
pub fn FeatEdit(props: FeatEditProps) -> Element {
    let store = use_context::<Store>();
    let all = store.feats;
    let feat_locked = props.feat;

    let feat = feat_locked.force_lock();

    // region: Signal
    let mut name = use_signal(|| feat.name.clone());
    let source = use_signal(|| feat.source.clone());
    let description = use_signal(|| feat.description.clone());
    let attributes = use_signal(|| feat.attributes.clone());
    let benefits = use_signal(|| feat.benefits.clone());

    drop(feat);

    let _ = use_effect(move || {
        let mut feat = feat_locked.force_lock();

        feat.name = name();
        feat.source = source();
        feat.description = description();
        feat.attributes = attributes();
        feat.benefits = benefits();
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

                Pair { name: "Source", align: true, grid: true,
                    SourceInputSignal { source }
                }
            }
            br {}

            h2 { "Description" }
            DescriptionInputSignal { description }

            h2 { "Attributes" }
            AttributesInputSignal { attributes }

            br {}

            StringListSignal { name: "Benefits", list: benefits }
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
