use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn Nav() -> Element {
    rsx! {
        nav { class: "flex w-full p-4 py-2 md:border-r border-b md:border-b-0 h-fit md:h-full md:w-fit",
            Navbar {}
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        ul { class: "flex gap-4 flex-row md:flex-col",
            li {
                Link { to: Routes::Home {}, "Home" }
            }
            li {
                Link { to: Routes::Items {}, "Items" }
            }
        }
    }
}

#[component]
pub fn Breadcrumbs() -> Element {
    let route: Routes = use_route();

    let segments = route.segments();

    rsx! {
        if let Some(segments) = segments {
            nav { class: "p-4 py-2",
                ul { class: "flex flex-row gap-2",
                    for (idx , segment) in segments.iter().enumerate() {
                        li {
                            if idx == segments.len() - 1 {
                                span { class: "text-gray-500", "{segment.name}" }
                            } else {
                                Link { to: segment.href.clone(), "{segment.name}" }
                            }
                        }

                        if idx < segments.len() - 1 {
                            span { class: "text-gray-500", "/ " }
                        }
                    }
                }
            }
        }
    }
}
