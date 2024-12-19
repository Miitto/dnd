use dioxus::prelude::*;

mod home;
mod items;

use crate::layouts::*;
use crate::Capitalize;
use crate::PageNotFound;
use home::Home;
use items::*;

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Routes {
    #[layout(NavLayout)]
    #[route("/")]
    Home {},
    #[route("/items")]
    Items {},
    #[route("/items/weapons")]
    Weapons {},
    #[route("/items/weapons/:id")]
    Weapon { id: String },
    #[end_layout]
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

#[derive(Debug)]
pub struct Segment {
    pub name: String,
    pub href: NavigationTarget,
}

impl Routes {
    pub fn segments(&self) -> Option<Vec<Segment>> {
        Some(match self {
            Routes::Home {} => vec![Segment {
                name: "Home".to_string(),
                href: Routes::Home {}.into(),
            }],
            Routes::Items {} => vec![Segment {
                name: "Items".to_string(),
                href: Routes::Items {}.into(),
            }],
            Routes::Weapons {} => vec![
                Segment {
                    name: "Items".to_string(),
                    href: Routes::Items {}.into(),
                },
                Segment {
                    name: "Weapons".to_string(),
                    href: Routes::Weapons {}.into(),
                },
            ],
            Routes::Weapon { id } => vec![
                Segment {
                    name: "Items".to_string(),
                    href: Routes::Items {}.into(),
                },
                Segment {
                    name: "Weapons".to_string(),
                    href: Routes::Weapons {}.into(),
                },
                Segment {
                    name: id.to_string().capitalize(),
                    href: Routes::Weapon { id: id.to_string() }.into(),
                },
            ],
            _ => return None,
        })
    }
}
