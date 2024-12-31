use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;

#[component]
pub fn SpellList(id: String) -> Element {
    let store = use_context::<Store>();
    let store = store.spell_lists;

    let list = use_memo(move || store.get(&id));

    rsx! {
        if let Some(list) = list() {
            h1 { class: "underline", "{list.name} Spell List" }
            ul { class: "list-disc pl-6",
                for item in list.spells.iter() {
                    li { key: item.name,
                        Link {
                            to: Routes::Spell {
                                id: item.name.to_string(),
                            },
                            "{item.name}"
                        }
                    }
                }
            }
        } else {
        "Can't find spell list"
    }
    }
}
