use std::{cell::RefCell, collections::HashMap};

use dioxus::prelude::*;
use types::meta::Description;

use crate::components::view::Pair;

#[component]
pub fn TextArea(
    value: ReadOnlySignal<String>,
    oninput: Option<Callback<String>>,
    onchange: Option<Callback<String>>,
) -> Element {
    rsx! {
        textarea {
            class: "w-full resize-none h-fit max-h-[50svh] min-h-40",
            value,
            oninput: move |e| {
                if let Some(cb) = oninput.as_ref() {
                    cb.call(e.value())
                }
            },
            onchange: move |e| {
                if let Some(cb) = onchange.as_ref() {
                    cb.call(e.value())
                }
            },
        }
    }
}

#[component]
pub fn TextAreaSignal(value: Signal<String>) -> Element {
    rsx! {
        TextArea { value: value(), oninput: move |v| value.set(v), onchange: None }
    }
}

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
    name: String,
    description: Description,
    list: Signal<HashMap<String, Description>>,
) -> Element {
    let mut name = use_signal(|| name);
    let desc = use_signal(|| description);

    let l = RefCell::new(list());
    let l_set = l.clone();

    use_effect(move || {
        *(l_set.borrow_mut()) = list();
    });

    use_effect(move || {
        l.borrow_mut().insert(name(), desc());
        list.set(l.borrow().clone());
    });

    rsx! {
        div { class: "flex flex-col gap-y-2",
            Pair { name: "Name", align: true,
                input {
                    value: name,
                    oninput: move |e| name.set(e.value()),
                    onchange: move |_| {
                        let mut l = list();
                        if let Some(desc) = l.remove(&name()) {
                            l.insert(name(), desc);
                            list.set(l);
                        }
                    },
                }
            }
            label {
                b { "Description" }
            }
            DescriptionInputSignal { description: desc }
        }
    }
}

#[component]
pub fn NameDescriptionListSignal(list: Signal<HashMap<String, Description>>) -> Element {
    let mut new_name = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col gap-y-4",
            for (idx , (name , description)) in list().into_iter().enumerate() {
                NameDescription { name, description, list }
                if (idx + 1) < list().len() {
                    br {}
                }
            }
            Pair { name: "Add", align: true,
                input {
                    value: "{new_name}",
                    onchange: move |e| {
                        if e.value().is_empty() {
                            return;
                        }
                        let mut l = list();
                        l.insert(e.value(), Description::default());
                        list.set(l);
                        new_name.set("".to_string());
                    },
                }
            }
        }
    }
}
