use crate::components::info::PairLi;
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
                    PairLi { name: "Age", "{race.age}" }
                    PairLi { name: "Alignment", "{race.alignment}" }
                    li {
                        p { class: "inline-flex gap-x-2",
                            b { "Size:" }
                            span { class: "inline-flex flex-wrap gap-x-2",
                                span { "{race.size.description}" }
                                span {
                                    "Your size is: "
                                    b { "{race.size.size}" }
                                }
                            }
                        }
                    }
                    PairLi { name: "Speed", "Your base walking speed is {race.speed} feet." }
                    PairLi { name: "Languages", "{race.languages}" }
                    hr {}
                    for (key , val) in &race.unique {
                        PairLi { name: "{key}", "{val}" }
                    }

                    for table in race.tables.iter() {
                        hr {}

                        table { class: "border w-full",
                            thead { class: "border-b",
                                tr { class: "*:px-2 *:py-1 *:text-left",
                                    for header in table[0].iter() {
                                        th { "{header}" }
                                    }
                                }
                            }
                            tbody {
                                for row in table[1..].iter() {
                                    tr { class: "*:px-2 *:py-1 even:bg-muted",
                                        for cell in row.iter() {
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
