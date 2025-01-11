use dioxus::prelude::*;
use types::stores::Store;

use crate::{components::view::Description, routes::Routes};

#[component]
pub fn Feat(id: String) -> Element {
    let store = use_context::<Store>();
    let store = store.feats;
    let feat = store.get_clone(&id);

    rsx! {
        if let Some(feat) = feat {
            span { class: "w-full inline-flex justify-between items-center",
                h1 { "{feat.name}" }
                if cfg!(debug_assertions) {
                    Link {
                        to: Routes::FeatEdit {
                            id: feat.name.to_owned(),
                        },
                        "Edit"
                    }
                }
            }
            div { class: "flex flex-col",
                Description { description: feat.description }

                if !feat.attributes.is_empty() {
                    br {}
                    h2 { "You gain the following attributes:" }
                    ul { class: "list-disc pl-6",
                        for (attr , value) in feat.attributes.iter() {
                            li { key: attr,
                                p { "{attr}: {value}" }
                            }
                        }
                    }
                }

                if !feat.benefits.is_empty() {
                    br {}
                    h2 { "You gain the following benefits:" }
                    ul { class: "list-disc pl-6",
                        for benefit in feat.benefits.iter() {

                            li { key: benefit,
                                p { "{benefit}" }
                            }
                        }
                    }
                }
            }
        } else {
            "Feat not found"
        }
    }
}
