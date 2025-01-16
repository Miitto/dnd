use dioxus::prelude::*;

mod description;
pub use description::*;
use types::meta::Source;

use crate::components::view::Pair;

use super::Checkbox;

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
pub fn SourceInput(
    value: ReadOnlySignal<Source>,
    callback: Callback<Source>,
    fire_on_input: Option<bool>,
) -> Element {
    let fire_on_input = fire_on_input.unwrap_or(false);
    let mut official = use_signal(|| value().is_official());

    rsx! {
        span { class: "inline-flex items-center gap-2",
            Checkbox {
                name: "Official",
                checked: value().is_official(),
                onchange: move |v| {
                    let source = if v {
                        Source::Official(value.read().to_string())
                    } else {
                        Source::Homebrew(value.read().to_string())
                    };
                    official.set(v);
                    callback.call(source);
                },
            }
            Pair {
                name: "From",
                align: true,
                class: "inline-flex items-center gap-2 flex-grow",
                input {
                    class: "flex-grow",
                    value: value().to_string(),
                    oninput: move |e| {
                        if fire_on_input {
                            let source = if official() {
                                Source::Official(e.value())
                            } else {
                                Source::Homebrew(e.value())
                            };
                            callback.call(source)
                        }
                    },
                    onchange: move |e| {
                        if !fire_on_input {
                            let source = if official() {
                                Source::Official(e.value())
                            } else {
                                Source::Homebrew(e.value())
                            };
                            callback.call(source)
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn SourceInputSignal(source: Signal<Source>, fire_on_input: Option<bool>) -> Element {
    rsx! {
        SourceInput {
            value: source,
            callback: move |s| source.set(s),
            fire_on_input: fire_on_input.unwrap_or(false),
        }
    }
}
