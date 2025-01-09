use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;

pub mod edit;
pub mod spell_list;

#[component]
pub fn SpellLists() -> Element {
    let store = use_context::<Store>();
    let store = store.spell_lists;

    let all = use_memo(move || {
        let mut all = store.all_vec();

        all.sort_by_key(|r| r.name.to_string());

        all
    });

    let mut new_list_name = use_signal(String::new);

    rsx! {
        h1 { class: "underline", "Spell Lists" }
        ul { class: "list-disc pl-6",
            for item in all() {
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

        div { class: "mt-4 flex items-center gap-2",
            input {
                r#type: "text",
                value: new_list_name(),
                oninput: move |e| new_list_name.set(e.value().trim().to_string()),
            }
            Link {
                to: Routes::SpellListEdit {
                    id: new_list_name(),
                },
                "New Spell List"
            }
        }
    }
}
