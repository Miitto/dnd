use std::collections::HashMap;

use dioxus::prelude::*;
use types::background::Background;
use types::stores::Store;

use crate::routes::Routes;
use crate::Capitalize;

pub mod background;
pub mod edit;

#[component]
pub fn Backgrounds() -> Element {
    let store = use_context::<Store>();
    let background_store = store.backgrounds;

    let backgrounds_map = use_memo(move || {
        let mut backgrounds: Vec<Background> = background_store.all_vec();

        backgrounds.sort_by_key(|r| r.name.to_string());

        let mut map = HashMap::new();

        for background in backgrounds {
            if !map.contains_key(&background.category) {
                map.insert(background.category.clone(), vec![]);
            }

            let category = map.get_mut(&background.category).unwrap();

            category.push(background);
        }

        map
    });

    let mut new_background_name = use_signal(String::new);

    rsx! {
        h1 { class: "underline", "Backgrounds" }
        for (category , backgrounds) in backgrounds_map() {
            h2 { class: "text-lg font-semibold", "{category.capitalize()}" }
            ul { class: "list-disc pl-6",
                for background in backgrounds {
                    li { key: background.name,
                        Link {
                            to: Routes::Background {
                                id: background.name.to_string(),
                            },
                            "{background.name}"
                        }
                    }
                }
            }
        }
        if cfg!(debug_assertions) {
            hr {}
            div { class: "mt-4 flex items-center gap-2",
                input {
                    r#type: "text",
                    value: new_background_name(),
                    oninput: move |e| new_background_name.set(e.value().trim().to_string()),
                }
                Link {
                    to: Routes::BackgroundEdit {
                        id: new_background_name(),
                    },
                    "New Background"
                }
            }
        }
    }
}
