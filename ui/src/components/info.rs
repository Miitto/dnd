use dioxus::prelude::*;

#[component]
pub fn Pair(name: String, children: Element) -> Element {
    rsx! {
        p { class: "inline-flex gap-x-2",
            b { "{name}:" }
            {children}
        }
    }
}

#[component]
pub fn PairLi(name: String, children: Element) -> Element {
    rsx! {
        li {
            Pair { name, children }
        }
    }
}
