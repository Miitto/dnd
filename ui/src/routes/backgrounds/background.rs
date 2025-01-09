use crate::components::view::{Description, PairLi};
use dioxus::prelude::*;
use types::stores::Store;

#[component]
pub fn Background(id: String) -> Element {
    let store = use_context::<Store>();
    let background_store = store.backgrounds;
    let background = background_store.get_clone(&id);

    if let Some(background) = background {
        let skills = background.skill_string();
        let tools = background.tool_string();
        let equip = background.equip_string();

        rsx! {
            h1 { "{background.name}" }

            div { class: "flex flex-col",
                Description { description: background.description }

                ul { class: "list-disc pl-6",
                    PairLi { name: "Skill Proficiencies", {skills} }

                    PairLi { name: "Tool Proficiencies", {tools} }

                    PairLi { name: "Languages", "{background.languages}" }

                    PairLi { name: "Equipment", {equip} }
                }
            }
        }
    } else {
        rsx! { "background not found" }
    }
}
