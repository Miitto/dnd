use std::collections::HashMap;

use dioxus::prelude::*;
use types::background::Background;
use types::stores::Store;

use crate::routes::Routes;
use crate::Capitalize;

pub mod background;

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
    }
}
