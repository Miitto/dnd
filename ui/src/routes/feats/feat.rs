use dioxus::prelude::*;
use types::stores::Store;

#[component]
pub fn Feat(id: String) -> Element {
    let store = use_context::<Store>();
    let store = store.feats;
    let feat = store.get_clone(&id);

    rsx! {
        if let Some(feat) = feat {
            h1 { "{feat.name}" }
            div { class: "flex flex-col",
                for split in feat.description.lines() {
                    p { "{split}" }
                }

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
