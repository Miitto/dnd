use crate::{
    components::view::spell_list::{make_dyn_level_button, SpellListView},
    routes::Routes,
};
use dioxus::prelude::*;
use types::stores::Store;

#[component]
pub fn SpellList(id: String, page: u8) -> Element {
    let store = use_context::<Store>();
    let store = store.spell_lists;

    let list = {
        let id = id.clone();
        use_signal(move || store.get_arced(&id))
    };

    let level_button = make_dyn_level_button(id.clone(), page);

    rsx! {
        if let Some(list) = list() {
            span { class: "w-full inline-flex justify-between items-center",
                h1 { "{list.name} Spell List" }
                Link { to: Routes::SpellListEdit { id }, "Edit" }
            }
            br {}
            SpellListView { list, page, level_button }
        } else {
            "Can't find spell list"
        }
    }
}
