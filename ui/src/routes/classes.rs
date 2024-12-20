use dioxus::prelude::*;
use types::stores::Store;

use crate::routes::Routes;

pub mod class;

#[component]
pub fn Classes() -> Element {
    let store = use_context::<Store>();
    let class_store = store.classes;

    let classes = use_hook(|| {
        let mut class = class_store.all();

        class.sort_by_key(|r| r.name.to_string());

        class
    });

    rsx! {
        h1 { class: "underline", "Races" }
        ul { class: "list-disc pl-6",
            for class in classes {
                li { key: class.name,
                    Link {
                        to: Routes::Class {
                            id: class.name.to_string(),
                        },
                        "{class.name}"
                    }
                }
            }
        }
    }
}
