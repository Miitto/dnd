use dioxus::prelude::*;
use types::stores::Store;

#[component]
pub fn Race(id: String) -> Element {
    let store = use_context::<Store>();
    let race_store = store.races;
    let race = use_memo(move || race_store.get(&id));

    rsx! {
        if let Some(race) = race() {
            h1 { "{race.name}" }

            div { class: "flex flex-col",
                p { "{race.description}" }

                ul { class: "list-disc pl-6 pt-4 gap-y-2",
                    li {
                        p { class: "inline-flex gap-x-2",
                            b { "Ability Score Increase:" }
                            for asi in race.default_asi.iter() {
                                span { key: asi.attribute, "{asi.attribute}: {asi.change}" }
                            }
                        }
                    }
                    Pair { name: "Age", value: "{race.age}" }
                    Pair { name: "Alignment", value: "{race.alignment}" }
                    li {
                        p { class: "inline-flex gap-x-2",
                            b { "Size:" }
                            span { "{race.size.description}" }
                            span {
                                "Your size is: "
                                b { "{race.size.size}" }
                            }
                        }
                    }
                    Pair {
                        name: "Speed",
                        value: "Your base walking speed is {race.speed} feet.",
                    }
                    Pair { name: "Languages", value: "{race.languages}" }
                    hr {}
                    for (key , val) in &race.unique {
                        Pair { name: "{key}", value: "{val}" }
                    }

                    for table in race.tables.iter() {
                        hr {}

                        table { class: "border",
                            for (idx , row) in table.iter().enumerate() {
                                tr { class: "*:px-2 *:py-1 first:border-b even:bg-muted",
                                    for cell in row.iter() {
                                        if idx == 0 {
                                            th { class: "text-left", "{cell}" }
                                        } else {
                                            td { "{cell}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            "race not found"
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
