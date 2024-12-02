#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use dnd_types::{items::weapon::WeaponType, stores::Store, *};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Weapons { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new().with_custom_head(
        r#"<link rel="stylesheet" href="./assets/tailwind.css">
        <link rel="stylesheet" href="tailwind.css">"#
            .to_string(),
    );
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let store = Store::from_path("assets/resources").unwrap();
    dbg!(&store);

    use_context_provider(|| store);

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Weapons() -> Element {
    let mut melee_count = 0;
    let mut ranged_count = 0;

    let store = use_context::<Store>();
    let weapon_store = store.weapons;
    let lock_r = weapon_store.weapons.lock();

    if lock_r.is_err() {
        return rsx! {
            Link { to: Route::Home {}, "Return Home"}
            "Loading Failed"
        };
    }

    let lock = lock_r.unwrap();

    lock.iter().for_each(|el| match **el {
        WeaponType::Melee(_) => {
            melee_count += 1;
        }
    });

    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Melee Weapons: {melee_count}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Weapons {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { class: "text-red-100 bg-red", onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
