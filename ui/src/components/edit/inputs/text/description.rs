use dioxus::prelude::*;
use types::meta::Description;

use crate::components::{edit::TextArea, view::Pair};

#[component]
pub fn DescriptionInput(
    description: ReadOnlySignal<Description>,
    oninput: Option<Callback<Description>>,
    onchange: Option<Callback<Description>>,
) -> Element {
    let oninput = oninput.map(|cb| {
        {
            move |d: String| {
                let d = Description::from(d);
                cb.call(d);
            }
        }
        .super_into()
    });

    let onchange = onchange.map(|cb| {
        {
            move |d: String| {
                let d = Description::from(d);
                cb.call(d);
            }
        }
        .super_into()
    });

    let description_string = use_memo(move || description().to_string().trim().to_string());

    rsx! {
        TextArea { value: description_string, oninput, onchange }
    }
}

#[component]
pub fn DescriptionInputSignal(description: Signal<Description>) -> Element {
    rsx! {
        DescriptionInput { description: description(), onchange: move |d| description.set(d) }
    }
}

#[component]
fn NameDescription(
    name: ReadOnlySignal<String>,
    description: ReadOnlySignal<Description>,
    on_change_name: Callback<String>,
    on_change_description: Callback<Description>,
    on_remove: Callback,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-y-2",
            Pair { name: "Name", align: true,
                input {
                    value: name,
                    onchange: move |e| {
                        on_change_name.call(e.value());
                    },
                }
            }
            label {
                b { "Description" }
            }
            DescriptionInput { description, onchange: move |e| {
        on_change_description.call(e);
    } }
            button {
                class: "px-4 py-2 w-fit border rounded",
                onclick: move |_| {
                    on_remove.call(());
                },
                "Remove"
            }
        }
    }
}

#[component]
pub fn NameDescriptionListSignal(list: Signal<Vec<(String, Description)>>) -> Element {
    let mut new_name = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col gap-y-4",
            for (name , description) in list() {
                if list.first().is_some() && list.first().unwrap().0 != name {
                    br {}
                }
                {
                    let change_name = name.clone();
                    let change_description = name.clone();
                    let remove = name.clone();
                    rsx! {
                        NameDescription {
                            name,
                            description,
                            on_change_name: move |n| {
                                let idx = list.read().iter().position(|(k, _)| *k == change_name);
                                if let Some(idx) = idx {
                                    list.write()[idx].0 = n;
                                }
                            },
                            on_change_description: move |d| {
                                let idx = list.read().iter().position(|(k, _)| *k == change_description);
                                if let Some(idx) = idx {
                                    list.write()[idx].1 = d;
                                }
                            },
                            on_remove: move || {
                                list.write().retain(|(k, _)| *k != remove);
                            },
                        }
                    }
                }
            }
            Pair { name: "Add", align: true,
                input {
                    value: "{new_name}",
                    onchange: move |e| {
                        if e.value().is_empty() {
                            return;
                        }
                        list.push((e.value(), Description::default()));
                        new_name.set("".to_string());
                    },
                }
            }
        }
    }
}
