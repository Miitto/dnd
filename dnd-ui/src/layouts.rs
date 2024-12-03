use dioxus::prelude::*;

use crate::{nav::Navbar, routes::Routes};

#[component]
pub fn NavLayout() -> Element {
    rsx! {
        div {
            nav {
                Navbar {}
            }
            main {
                Outlet::<Routes> {}
            }
        }
    }
}
