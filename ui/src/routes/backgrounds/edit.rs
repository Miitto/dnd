use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::stores::Store;

use crate::components::edit::BackgroundEdit as BackgroundEditComponent;

#[component]
pub fn BackgroundEdit(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.backgrounds;

    let background_locked = {
        let background_locked = all.get(&id).unwrap_or_default();

        let background = background_locked.force_lock();

        if background.name != id {
            let mut clone = (*background).clone();
            clone.name = id.clone();
            let arc = Arc::new(Mutex::new(clone));
            all.store
                .lock()
                .expect("Failed to lock backgrounds when inserting new background")
                .insert(id.clone(), arc.clone());

            arc
        } else {
            drop(background);
            background_locked
        }
    };

    rsx! {
        BackgroundEditComponent { background: background_locked }
    }
}
