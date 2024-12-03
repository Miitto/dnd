use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        ul {
            li {
                Link {
                    to: Routes::Home {},
                    "Home"
                }
            },
            li {
                Link {
                    to: Routes::Items {},
                    "Items"
                }
            }
        }
    }
}
