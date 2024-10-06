use dnd::types::items::weapon::MeleeWeapon as MeleeWeaponT;
use leptos::{component, create_signal, view, IntoView, Params, SignalWith};
use leptos_router::{use_params, Outlet, Params, Route, A};

#[component(transparent)]
pub fn DatabaseRoutes() -> impl IntoView {
    view! {
        <Route path="database" view=Database>
            <Route path="melee-weapon" view=MeleeWeapon>
                <Route path=":name" view=MeleeWeaponPage />
                <Route path="" view=MeleeWeaponList />
            </Route>
            <Route path="" view=|| view! {
                <A href="/database/melee-weapon">Melee Weapons</A>
            } />
        </Route>
    }
}

#[component]
pub fn Database() -> impl IntoView {
    view! {
        <div>
            <h1>Database</h1>

        </div>
        <Outlet />
    }
}

#[component]
pub fn MeleeWeapon() -> impl IntoView {
    view! {
        <div>
            <h1>Melee Weapons</h1>
        </div>
        <Outlet />
    }
}

#[component]
pub fn MeleeWeaponList() -> impl IntoView {
    view! {
        <A href="/database/melee-weapon/rapier">Rapier</A>
    }
}

#[derive(Params, PartialEq)]
struct MeleeWeaponPageParams {
    name: Option<String>,
}

#[component]
pub fn MeleeWeaponPage() -> impl IntoView {
    let params = use_params::<MeleeWeaponPageParams>();

    let name = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.name.clone())
                .unwrap_or_default()
        })
    };

    let (weapon, set_weapon) = create_signal(Option::<MeleeWeaponT>::None);

    let has_name = move || name().is_some();

    view! {
            {move || {
                if has_name() {
                    view! {
                        <div>
                            <h1>{format!("Melee Weapon: {}", name().unwrap())}</h1>
                        </div>
                    }
                } else {
                    view! {
                        <div>
                            <p>Invalid Weapon</p>
                        </div>
                    }
                }
            }}
    }
}
