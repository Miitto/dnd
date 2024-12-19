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
                    Pair {
                        name: "Skill Proficiencies",
                        value: background.skill_string(),
                    }

                    Pair {
                        name: "Tool Proficiencies",
                        value: background.tool_string(),
                    }

                    Pair { name: "Languages", value: "{background.languages}" }

                    Pair { name: "Equipment", value: background.equip_string() }
                }
            }
        } else {
            "background not found"
        }
    }
}

#[component]
fn Pair(name: String, value: String) -> Element {
    rsx! {
        li {
            p {
                b { "{name}:" }
                " {value}"
            }
        }
    }
}
