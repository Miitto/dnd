use dioxus::prelude::*;

mod home;
mod items;

use crate::layouts::*;
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
