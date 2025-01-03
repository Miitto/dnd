use std::collections::HashMap;

use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;
use crate::Capitalize;

pub mod race;

#[component]
pub fn Races() -> Element {
    let store = use_context::<Store>();
    let race_store = store.races;

    let races_map = use_hook(|| {
        let mut races = race_store.all();

        races.sort_by_key(|r| r.name.to_string());

        let mut map = HashMap::new();

        for race in races {
            if !map.contains_key(&race.category) {
                map.insert(race.category.clone(), vec![]);
            }

            let category = map.get_mut(&race.category).unwrap();

            category.push(race);
        }

        map
    });

    rsx! {
        h1 { class: "underline", "Races" }
        for (category , races) in races_map {
            h2 { class: "text-lg font-semibold", "{category.capitalize()}" }
            ul { class: "list-disc pl-6",
                for race in races {
                    li { key: race.name,
                        Link {
                            to: Routes::Race {
                                id: race.name.to_string(),
                            },
                            "{race.name}"
                        }
                    }
                }
            }
        }
    }
}
