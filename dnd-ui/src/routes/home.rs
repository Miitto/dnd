use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Routes::Weapons {},
            "Go to weapons"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { class: "text-red-100 bg-red", onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
