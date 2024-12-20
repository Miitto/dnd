use dioxus::prelude::*;

#[component]
pub fn Pair(name: String, value: String) -> Element {
    rsx! {
        li {
            p {
                b { "{name}:" }
                " {value}"
            }
        }
    }
}
