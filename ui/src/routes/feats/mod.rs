use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;

pub mod edit;
pub mod feat;

#[component]
pub fn Feats() -> Element {
    let store = use_context::<Store>();
    let feat_store = store.feats;

    let feats = use_memo(move || {
        let mut all = feat_store.all_vec();

        all.sort_by_key(|r| r.name.to_string());

        all
    });

    let mut new_feat_name = use_signal(String::new);

    rsx! {
        h1 { class: "underline", "Feats" }
        ul { class: "list-disc pl-6",
            for feat in feats() {
                li { key: feat.name,
                    Link {
                        to: Routes::Feat {
                            id: feat.name.to_string(),
                        },
                        "{feat.name}"
                    }
                }
            }
        }
        if cfg!(debug_assertions) {
            hr {}
            div { class: "mt-4 flex items-center gap-2",
                input {
                    r#type: "text",
                    value: new_feat_name(),
                    oninput: move |e| new_feat_name.set(e.value().trim().to_string()),
                }
                Link {
                    to: Routes::FeatEdit {
                        id: new_feat_name(),
                    },
                    "New Feat"
                }
            }
        }
    }
}
