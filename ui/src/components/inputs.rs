use dioxus::prelude::*;
use types::common::{Damage, Dice};

use crate::components::info::Pair;

#[component]
pub fn StringList(name: String, list: Vec<String>, oninput: Callback<Vec<String>>) -> Element {
    let mut list = use_signal(move || list.clone());

    let mut value = use_signal(move || "".to_string());

    rsx! {
        div { class: "flex flex-col gap-2",
            div { class: "flex gap-2 flex-row",
                Pair { name: "{name}", class: "flex-grow",
                    input {
                        class: "flex-grow",
                        value,
                        onchange: move |e| {
                            if e.value().is_empty() {
                                return;
                            }
                            let mut l = list();
                            if l.contains(&e.value()) {
                                return;
                            }
                            l.push(e.value());
                            list.set(l);
                            oninput(list());
                            value.set("".to_string());
                        },
                    }
                }
            }
            div { class: "flex flex-col gap-2",
                for (idx , item) in list.iter().enumerate() {
                    div { class: "grid grid-cols-fr-auto gap-2",
                        input {
                            value: "{item}",
                            oninput: move |e| {
                                let mut l = list();
                                l[idx] = e.value();
                                list.set(l);
                                oninput(list());
                            },
                            onchange: move |e| {
                                if e.value().is_empty() {
                                    let mut l = list();
                                    l.remove(idx);
                                    list.set(l);
                                }
                            },
                        }
                        button {
                            onclick: move |_| {
                                let mut l = list();
                                l.remove(idx);
                                list.set(l);
                                oninput(list());
                            },
                            "Remove"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AttrDropdown(
    value: String,
    allow_none: Option<bool>,
    onchange: Callback<String>,
) -> Element {
    let mut value = use_signal(move || value.clone());
    let allow_none = allow_none.unwrap_or_default();

    rsx! {
        select {
            value,
            onchange: move |e| {
                value.set(e.value());
                onchange.call(e.value());
            },
            if allow_none {
                option { value: "", "None" }
            }
            option { value: "str", "Strength" }
            option { value: "dex", "Dexterity" }
            option { value: "con", "Constitution" }
            option { value: "int", "Intelligence" }
            option { value: "wis", "Wisdom" }
            option { value: "cha", "Charisma" }
        }
    }
}

#[component]
pub fn DiceInput(value: Dice, grid: Option<bool>, onchange: Callback<Dice>) -> Element {
    let mut dice = use_signal(move || value);

    let display = if grid.unwrap_or_default() {
        "grid grid-cols-subgrid col-span-4"
    } else {
        "inline-flex"
    };

    rsx! {
        div { class: "{display} gap-x-2 items-center",
            input {
                r#type: "number",
                min: 1,
                value: "{dice().count}",
                onchange: move |e| {
                    let mut d = dice();
                    d.count = e.value().parse().unwrap_or(0);
                    dice.set(d);
                    onchange.call(dice());
                },
            }
            select {
                value: "{dice().sides}",
                onchange: move |e| {
                    let mut d = dice();
                    d.sides = e.value().parse().unwrap_or(4);
                    dice.set(d);
                    onchange.call(dice());
                },
                option { value: "4", "d4" }
                option { value: "6", "d6" }
                option { value: "8", "d8" }
                option { value: "10", "d10" }
                option { value: "12", "d12" }
                option { value: "20", "d20" }
                option { value: "100", "d100" }
            }
            span { class: "h-fit", " + " }
            input {
                r#type: "number",
                value: "{dice().modifier.unwrap_or_default()}",
                onchange: move |e| {
                    let mut d = dice();
                    d.modifier = e.value().parse().ok().filter(|&x| x != 0);
                    dice.set(d);
                    onchange.call(dice());
                },
            }
        }
    }
}

#[component]
pub fn DamageInput(value: Damage, grid: Option<bool>, onchange: Callback<Damage>) -> Element {
    let mut damage = use_signal(move || value.clone());

    let display = if grid.unwrap_or_default() {
        "grid grid-cols-subgrid col-span-5"
    } else {
        "inline-flex"
    };

    rsx! {
        span { class: "{display} gap-x-2",
            DiceInput {
                value: damage().dice,
                grid,
                onchange: move |d| {
                    let mut dmg = damage();
                    dmg.dice = d;
                    damage.set(dmg);
                    onchange.call(damage());
                },
            }
            input {
                value: "{damage().damage_type}",
                onchange: move |e| {
                    let mut dmg = damage();
                    dmg.damage_type.set(e.value());
                    damage.set(dmg);
                    onchange.call(damage());
                },
            }
        }
    }
}

#[component]
pub fn MultiDamageInput(value: Vec<Damage>, onchange: Callback<Vec<Damage>>) -> Element {
    let mut damages = use_signal(move || value.clone());

    rsx! {
        div { class: "flex flex-col gap-2 items-end",
            div { class: "grid grid-cols-[1fr_1fr_auto_1fr_1fr_auto] gap-2 w-full",
                for (idx , dmg) in damages.iter().enumerate() {
                    DamageInput {
                        grid: true,
                        value: dmg.clone(),
                        onchange: move |d| {
                            let mut dmg = damages();
                            dmg[idx] = d;
                            damages.set(dmg);
                            onchange.call(damages());
                        },
                    }
                    button {
                        class: "px-4 py-2 text-xl",
                        onclick: move |_| {
                            let mut dmg = damages();
                            dmg.remove(idx);
                            damages.set(dmg);
                            onchange.call(damages());
                        },
                        "-"
                    }
                }
            }
            button {
                class: "w-fit px-4 py-2 text-xl",
                onclick: move |_| {
                    damages
                        .push(Damage {
                            dice: Dice {
                                count: 1,
                                sides: 4,
                                modifier: None,
                            },
                            damage_type: "".into(),
                        });
                    onchange.call(damages());
                },
                "+"
            }
        }
    }
}
