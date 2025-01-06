use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;

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
    }
}
