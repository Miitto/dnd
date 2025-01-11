use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::stores::Store;

use crate::components::edit::RaceEdit as RaceEditComponent;

#[component]
pub fn RaceEdit(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.races;

    let race_locked = {
        let race_locked = all.get(&id).unwrap_or_default();

        let race = race_locked.force_lock();

        if race.name != id {
            let mut clone = (*race).clone();
            clone.name = id.clone();
            let arc = Arc::new(Mutex::new(clone));
            all.store
                .lock()
                .expect("Failed to lock races when inserting new race")
                .insert(id.clone(), arc.clone());

            arc
        } else {
            drop(race);
            race_locked
        }
    };

    rsx! {
        RaceEditComponent { race: race_locked }
    }
}
