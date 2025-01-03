use dioxus::prelude::*;

use crate::components::info::Pair;

#[component]
pub fn StringList(name: String, list: Vec<String>, oninput: Callback<Vec<String>>) -> Element {
    let mut list = use_signal(move || list.clone());

    let mut value = use_signal(move || "".to_string());

    rsx! {
        div { class: "flex flex-col gap-2",
            div { class: "flex gap-2 flex-row",
                Pair { name: "{name}", class: "flex-grow",
                    input {
                        class: "flex-grow",
                        value,
                        onchange: move |e| {
                            if e.value().is_empty() {
                                return;
                            }
                            let mut l = list();
                            if l.contains(&e.value()) {
                                return;
                            }
                            l.push(e.value());
                            list.set(l);
                            oninput(list());
                            value.set("".to_string());
                        },
                    }
                }
            }
            div {
                for (idx , item) in list.iter().enumerate() {
                    div { class: "grid grid-cols-fr-auto gap-2",
                        input {
                            value: "{item}",
                            oninput: move |e| {
                                let mut l = list();
                                l[idx] = e.value();
                                list.set(l);
                                oninput(list());
                            },
                            onchange: move |e| {
                                if e.value().is_empty() {
                                    let mut l = list();
                                    l.remove(idx);
                                    list.set(l);
                                }
                            },
                        }
                        button {
                            onclick: move |_| {
                                let mut l = list();
                                l.remove(idx);
                                list.set(l);
                                oninput(list());
                            },
                            "Remove"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AttrDropdown(
    value: String,
    allow_none: Option<bool>,
    onchange: Callback<String>,
) -> Element {
    let mut value = use_signal(move || value.clone());
    let allow_none = allow_none.unwrap_or_default();

    rsx! {
        select {
            value,
            onchange: move |e| {
                value.set(e.value());
                onchange.call(e.value());
            },
            if allow_none {
                option { value: "", "None" }
            }
            option { value: "str", "Strength" }
            option { value: "dex", "Dexterity" }
            option { value: "con", "Constitution" }
            option { value: "int", "Intelligence" }
            option { value: "wis", "Wisdom" }
            option { value: "cha", "Charisma" }
        }
    }
}
