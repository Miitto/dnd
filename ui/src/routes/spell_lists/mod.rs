use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;

pub mod spell_list;

#[component]
pub fn SpellLists() -> Element {
    let store = use_context::<Store>();
    let store = store.spell_lists;

    let all = use_hook(|| {
        let mut all = store.all();

        all.sort_by_key(|r| r.name.to_string());

        all
    });

    rsx! {
        h1 { class: "underline", "Spell Lists" }
        ul { class: "list-disc pl-6",
            for item in all {
                li { key: item.name,
                    Link {
                        to: Routes::SpellList {
                            id: item.name.to_string(),
                            page: 0,
                        },
                        "{item.name}"
                    }
                }
            }
        }
    }
}
