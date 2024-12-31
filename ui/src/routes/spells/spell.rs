use dioxus::prelude::*;
use types::stores::Store;

#[component]
pub fn Spell(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.spells;

    let spell = use_memo(move || all.get(&id));

    rsx! {}
}
