use crate::components::info::PairLi;
use dioxus::prelude::*;
use types::stores::Store;

#[component]
pub fn Background(id: String) -> Element {
    let store = use_context::<Store>();
    let background_store = store.backgrounds;
    let background = use_memo(move || background_store.get(&id));

    rsx! {
        if let Some(background) = background() {
            h1 { "{background.name}" }

            div { class: "flex flex-col",
                for split in background.description.split('\n') {
                    p { "{split}" }
                }

                ul { class: "list-disc pl-6",
                    PairLi { name: "Skill Proficiencies", {background.skill_string()} }

                    PairLi { name: "Tool Proficiencies", {background.tool_string()} }

                    PairLi { name: "Languages", "{background.languages}" }

                    PairLi { name: "Equipment", {background.equip_string()} }
                }
            }
        } else {
            "background not found"
        }
    }
}
