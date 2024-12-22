use std::sync::Arc;

use dioxus::prelude::*;
use types::{classes::Class as ClassT, stores::Store};

use crate::routes::Routes;

mod class;
mod subclass;

pub use class::Class;
pub use subclass::Subclass;

#[component]
pub fn Classes() -> Element {
    let store = use_context::<Store>();
    let class_store = store.classes;

    let classes = use_memo(move || {
        let mut class = class_store.all();

        class.sort_by_key(|r| r.name.to_string());

        class
    });

    rsx! {
        h1 { class: "underline", "Races" }
        ul { class: "list-disc pl-6",
            for class in classes() {
                li { key: class.name,
                    Link {
                        to: Routes::Class {
                            id: class.name.to_string(),
                        },
                        "{class.name}"
                    }
                    if !class.subclasses.is_empty() {
                        SubclassList { class }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SubclassList(class: Arc<ClassT>) -> Element {
    let class_c = Arc::clone(&class);
    let subclasses = use_memo(move || {
        let mut subclasses = class_c
            .subclasses
            .options
            .values()
            .map(|s| s.name.clone())
            .collect::<Vec<_>>();

        subclasses.sort();

        subclasses
    });

    rsx! {
        ul { class: "list-circle pl-6",
            for subclass in subclasses() {
                li { key: subclass,
                    Link {
                        to: Routes::Subclass {
                            class_id: class.name.to_string(),
                            subclass_id: subclass.to_string(),
                        },
                        "{subclass}"
                    }
                }
            }
        }
    }
}
