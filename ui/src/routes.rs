use dioxus::prelude::*;

pub mod backgrounds;
pub mod classes;
mod feats;
mod home;
mod items;
pub mod races;
pub mod spell_lists;
pub mod spells;

use crate::layouts::*;
use crate::Capitalize;
use crate::PageNotFound;
use home::Home;
use items::*;

use backgrounds::{background::Background, edit::BackgroundEdit, Backgrounds};
use classes::{Class, Classes, Subclass};
use feats::{edit::FeatEdit, feat::Feat, Feats};
use races::{edit::RaceEdit, race::Race, Races};
use spell_lists::{edit::SpellListEdit, spell_list::SpellList, SpellLists};
use spells::{Spell, SpellEdit};

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Routes {
    #[layout(NavLayout)]
    #[route("/")]
    Home {},
    #[nest("/items")]
        #[route("/")]
        Items {},
        #[nest("/weapons")]
            #[route("/")]
            Weapons {},
            #[route("/:id")]
            Weapon { id: String },
        #[end_nest]
    #[end_nest]
    #[nest("/races")]
        #[route("/")]
        Races {},
        #[route("/:id")]
        Race { id: String },
        #[route("/:id/edit")]
        RaceEdit { id: String },
    #[end_nest]
    #[nest("/backgrounds")]
        #[route("/")]
        Backgrounds {},
        #[route("/:id")]
        Background { id: String },
        #[route("/:id/edit")]
        BackgroundEdit { id: String },
    #[end_nest]
    #[nest("/classes")]
        #[route("/")]
        Classes {},
        #[route("/:id")]
        Class { id: String },
        #[route("/:class_id/:subclass_id")]
        Subclass { class_id: String, subclass_id: String },
    #[end_nest]
    #[nest("/feats")]
        #[route("/")]
        Feats {},
        #[route("/:id")]
        Feat { id: String },
        #[route("/:id/edit")]
        FeatEdit { id: String },
    #[end_nest]
    #[nest("/spell_lists")]
        #[route("/")]
        SpellLists {},
        #[redirect("/:id", |id: String| Routes::SpellList { id, page: 0 })]
        #[route("/:id/:page")]
        SpellList { id: String, page: u8 },
        #[route("/:id/edit")]
        SpellListEdit { id: String },
    #[end_nest]
    #[nest("/spells")]
        //#[route("/")]
        //Spells {},
        #[route("/:id")]
        Spell { id: String },
        #[route("/:id/edit")]
        SpellEdit { id: String },
    #[end_nest]
    #[end_layout]
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

impl Routes {
    pub fn segments(&self) -> Option<Vec<Segment>> {
        Some(match self {
            Routes::Home {} => vec![self.as_segment("Home")],
            Routes::Items {} => vec![self.as_segment("Items")],
            Routes::Weapons {} => Routes::Items {}.add_segment(self.as_segment("Weapons")),
            Routes::Weapon { id } => {
                Routes::Weapons {}.add_segment(self.as_segment(id.capitalize()))
            }
            Routes::Races {} => vec![self.as_segment("Races")],
            Routes::Race { id } => Routes::Races {}.add_segment(self.as_segment(id.capitalize())),
            Routes::RaceEdit { id } => {
                Routes::Race { id: id.clone() }.add_segment(self.as_segment("Edit"))
            }
            Routes::Backgrounds {} => vec![self.as_segment("Backgrounds")],
            Routes::Background { id } => {
                Routes::Backgrounds {}.add_segment(self.as_segment(id.capitalize()))
            }
            Routes::BackgroundEdit { id } => {
                Routes::Background { id: id.clone() }.add_segment(self.as_segment("Edit"))
            }
            Routes::Classes {} => vec![self.as_segment("Classes")],
            Routes::Class { id } => {
                Routes::Classes {}.add_segment(self.as_segment(id.capitalize()))
            }
            Routes::Subclass {
                class_id,
                subclass_id,
            } => Routes::Class {
                id: class_id.clone(),
            }
            .add_segment(self.as_segment(subclass_id.capitalize())),
            Routes::Feats {} => vec![self.as_segment("Feats")],
            Routes::Feat { id } => Routes::Feats {}.add_segment(self.as_segment(id.capitalize())),
            Routes::FeatEdit { id } => {
                Routes::Feat { id: id.clone() }.add_segment(self.as_segment("Edit"))
            }
            Routes::SpellLists {} => vec![self.as_segment("Spell Lists")],
            Routes::SpellList { id, .. } => {
                Routes::SpellLists {}.add_segment(self.as_segment(id.capitalize()))
            }
            Routes::SpellListEdit { id } => Routes::SpellList {
                id: id.clone(),
                page: 0,
            }
            .add_segment(self.as_segment("Edit")),
            Routes::Spell { id } => vec![self.as_segment(id.capitalize())],
            Routes::SpellEdit { id } => {
                Routes::Spell { id: id.clone() }.add_segment(self.as_segment("Edit"))
            }
            _ => return None,
        })
    }

    fn add_segment(&self, seg: Segment) -> Vec<Segment> {
        let mut segments = self.segments().unwrap();
        segments.push(seg);

        segments
    }

    fn as_segment<T: AsRef<str>>(&self, name: T) -> Segment {
        Segment {
            name: name.as_ref().to_string(),
            href: self.clone().into(),
        }
    }
}

#[derive(Debug)]
pub struct Segment {
    pub name: String,
    pub href: NavigationTarget,
}
