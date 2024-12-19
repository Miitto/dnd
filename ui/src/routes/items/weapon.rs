use dioxus::prelude::*;
use types::items::properties::EffectType;
use types::stores::Store;

#[component]
pub fn Weapon(id: String) -> Element {
    let store = use_context::<Store>();
    let weapon_store = store.weapons;
    let weapon = use_memo(move || weapon_store.find_weapon(&id));

    rsx! {
        if let Some(weapon) = weapon() {
            h1 { "{weapon.name}" }

            div { class: "flex",
                div { class: "w-1/2",
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
                            h3 { "{property.name}" }
                            if let Some(description) = &property.description {
                                p { "{description}" }
                            }
                            h4 { "Effects" }
                            for effect in &property.effects {
                                hr {}
                                span {
                                    "Optional: "
                                    input {
                                        r#type: "checkbox",
                                        disabled: true,
                                        checked: effect.optional,
                                    }
                                }
                                if let Some(when) = &effect.when {
                                    p { "When: {when}" }
                                }
                                p {
                                    match &effect.effect_type {
                                        EffectType::Damage(dmg) => format!("Damage: {}", dmg),
                                        EffectType::Attribute(attr) => attr.to_string(),
                                    }
                                }
                                hr {}
                            }
                        }
                    }
                    br {}
                    p {
                        b { "Subtype:" }
                        " {weapon.subtype.join(\", \")}"
                    }
                }
            }
        } else {
            "Weapon not found"
        }
    }
}
