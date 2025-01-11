use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::stores::Store;

use crate::components::edit::FeatEdit as FeatEditComponent;

#[component]
pub fn FeatEdit(id: String) -> Element {
    let store = use_context::<Store>();

    let all = store.feats;

    let feat_locked = {
        let feat_locked = all.get(&id).unwrap_or_default();

        let feat = feat_locked.force_lock();

        if feat.name != id {
            let mut clone = (*feat).clone();
            clone.name = id.clone();
            let arc = Arc::new(Mutex::new(clone));
            all.store
                .lock()
                .expect("Failed to lock feats when inserting new feat")
                .insert(id.clone(), arc.clone());

            arc
        } else {
            drop(feat);
            feat_locked
        }
    };

    rsx! {
        FeatEditComponent { feat: feat_locked }
    }
}
