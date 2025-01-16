use crate::{
    components::view::{Description, PairLi},
    routes::Routes,
};
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
            span { class: "w-full inline-flex justify-between items-center",
                h1 { "{background.name}" }
                if cfg!(debug_assertions) {
                    Link {
                        to: Routes::BackgroundEdit {
                            id: background.name.to_owned(),
                        },
                        "Edit"
                    }
                }
            }

            div { class: "flex flex-col",
                Description { description: background.description }

                br {}

                ul { class: "flex flex-col list-disc pl-6 gap-y-2",
                    if !skills.is_empty() {
                        PairLi { name: "Skill Proficiencies", {skills} }
                    }

                    if !tools.is_empty() {
                        PairLi { name: "Tool Proficiencies", {tools} }
                    }

                    if !background.languages.is_empty() {
                        PairLi { name: "Languages", "{background.languages}" }
                    }

                    if !equip.is_empty() {
                        PairLi { name: "Equipment", {equip} }
                    }
                }
            }
            h2 { "Features" }
            br {}
            div { class: "flex flex-col gap-y-2",
                for feature in background.features {
                    h3 { "{feature.name}" }
                    Description { description: feature.description }
                    br {}
                }
            }
        }
    } else {
        rsx! { "background not found" }
    }
}
