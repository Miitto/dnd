use dioxus::prelude::*;
use types::items::properties::{EffectType, Property};
use types::stores::Store;

#[component]
pub fn Weapon(id: String) -> Element {
    let store = use_context::<Store>();
    let weapon_store = store.weapons;
    let weapon = use_memo(move || weapon_store.get(&id));

    rsx! {
        if let Some(weapon) = weapon() {
            h1 { "{weapon.name}" }

            div { class: "flex flex-col",
                p {
                    b { "Damage:" }
                    " {weapon.damage}"
                }
                p {
                    b { "Rarity:" }
                    " {weapon.rarity}"
                }
                p {
                    b { "Weight:" }
                    " {weapon.weight}"
                }
                h2 { "Properties" }
                div { class: "pl-2",
                    for property in weapon.properties.iter() {
                        WeaponProperty { property: property.clone() }
                    }
                }
                br {}
                p {
                    b { "Subtype:" }
                    " {weapon.subtype.join(\", \")}"
                }
            }
        } else {
            "Weapon not found"
        }
    }
}

#[component]
fn WeaponProperty(property: Property) -> Element {
    rsx! {
        h3 { "{property.name}" }
        if let Some(description) = &property.description {
            p { "{description}" }
        }
        h4 { "Effects" }
        for effect in &property.effects {
            hr {}
            span { class: "flex flex-wrap gap-x-4 gap-y-2",
                span { class: "flex gap-x-2",
                    b { "Optional:" }
                    input {
                        r#type: "checkbox",
                        disabled: true,
                        checked: effect.optional,
                    }
                }
                if let Some(when) = &effect.when {
                    p {
                        b { "When:" }
                        " {when}"
                    }
                }
            }
            p {
                PropertyEffectType { effect_type: effect.effect_type.clone() }
            }
            hr {}
        }
    }
}

#[component]
fn PropertyEffectType(effect_type: EffectType) -> Element {
    rsx! {
        match effect_type {
            EffectType::Damage(dmg) => rsx! {
                b { "Damage:" }
                " {dmg}"
            },
            EffectType::Attribute(attr) => rsx! {
            "{attr.to_string()}"
            },
        }
    }
}
