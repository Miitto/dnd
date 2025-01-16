use dioxus::prelude::*;
use types::mechanics::{Attributes, Damage, Dice, Size};

use crate::components::view::Pair;

mod text;
pub use text::*;

#[component]
pub fn Checkbox(name: String, checked: ReadOnlySignal<bool>, onchange: Callback<bool>) -> Element {
    rsx! {
        fieldset { class: "inline-flex gap-2 items-center",
            input {
                r#type: "checkbox",
                checked,
                onchange: move |e| {
                    let checked = e.checked();
                    onchange.call(checked);
                },
            }
            label { "{name}" }
        }
    }
}

#[component]
pub fn CheckboxSignal(name: String, checked: Signal<bool>) -> Element {
    rsx! {
        Checkbox { name, checked: checked(), onchange: move |c| checked.set(c) }
    }
}

#[component]
pub fn StringList(
    name: String,
    list: ReadOnlySignal<Vec<String>>,
    oninput: Callback<Vec<String>>,
) -> Element {
    let mut value = use_signal(move || "".to_string());

    rsx! {
        div { class: "flex flex-col gap-2",
            h2 { "{name}" }
            div { class: "flex flex-col gap-2",
                for (idx , item) in list.iter().enumerate() {
                    div { class: "grid grid-cols-fr-auto gap-2",
                        input {
                            value: "{item}",
                            oninput: move |e| {
                                let mut l = list();
                                l[idx] = e.value();
                                oninput(l);
                            },
                            onchange: move |e| {
                                if e.value().is_empty() {
                                    let mut l = list();
                                    l.remove(idx);
                                    oninput(l);
                                }
                            },
                        }
                        button {
                            onclick: move |_| {
                                let mut l = list();
                                l.remove(idx);
                                oninput(l);
                            },
                            "Remove"
                        }
                    }
                }
            }
            div { class: "flex gap-2 flex-row",
                Pair { name: "New", class: "flex flex-grow", align: true,
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
                            oninput.call(l);
                            value.set("".to_string());
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn StringListSignal(list: Signal<Vec<String>>, name: String) -> Element {
    rsx! {
        StringList { list: list(), name, oninput: move |l| list.set(l) }
    }
}

#[component]
pub fn AttrDropdown(
    value: ReadOnlySignal<String>,
    allow_none: Option<bool>,
    onchange: Callback<String>,
) -> Element {
    let allow_none = allow_none.unwrap_or_default();

    rsx! {
        select {
            value,
            onchange: move |e| {
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
pub fn AttributesInput(
    value: ReadOnlySignal<Attributes>,
    onchange: Callback<Attributes>,
) -> Element {
    macro_rules! pair {
        ($text:literal, $name:ident) => {
            rsx! {
                Pair {
                    name: $text,
                    grid: true,
                    align: true,
                    input {
                        r#type: "number",
                        value: value().$name,
                        onchange: move |e| {
                        let mut a = value();
                            a.$name = e.value().parse().unwrap_or_default();
                            onchange.call(a);
                        },
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "grid grid-cols-auto-fr gap-y-2",
            {pair!("Strength", strength)}
            {pair!("Dexterity", dexterity)}
            {pair!("Constitution", constitution)}
            {pair!("Intelligence", intelligence)}
            {pair!("Wisdom", wisdom)}
            {pair!("Charisma", charisma)}
        }
    }
}

#[component]
pub fn AttributesInputSignal(attributes: Signal<Attributes>) -> Element {
    rsx! {
        AttributesInput { value: attributes(), onchange: move |a| attributes.set(a) }
    }
}

#[component]
pub fn DiceInput(
    value: ReadOnlySignal<Dice>,
    grid: Option<bool>,
    onchange: Callback<Dice>,
) -> Element {
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
                value: "{value().count}",
                onchange: move |e| {
                    let mut d = value();
                    d.count = e.value().parse().unwrap_or(0);
                    onchange.call(d);
                },
            }
            select {
                value: "{value().sides}",
                onchange: move |e| {
                    let mut d = value();
                    d.sides = e.value().parse().unwrap_or(4);
                    onchange.call(d);
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
                value: "{value().modifier.unwrap_or_default()}",
                onchange: move |e| {
                    let mut d = value();
                    d.modifier = e.value().parse().ok().filter(|&x| x != 0);
                    onchange.call(d);
                },
            }
        }
    }
}

#[component]
pub fn DamageInput(
    value: ReadOnlySignal<Damage>,
    grid: Option<bool>,
    onchange: Callback<Damage>,
) -> Element {
    let display = if grid.unwrap_or_default() {
        "grid grid-cols-subgrid col-span-5"
    } else {
        "inline-flex"
    };

    rsx! {
        span { class: "{display} gap-x-2",
            DiceInput {
                value: value().dice,
                grid,
                onchange: move |d| {
                    let mut dmg = value();
                    dmg.dice = d;
                    onchange.call(value());
                },
            }
            input {
                value: "{value().damage_type}",
                onchange: move |e| {
                    let mut dmg = value();
                    dmg.damage_type.set(e.value());
                    onchange.call(value());
                },
            }
        }
    }
}

#[component]
pub fn MultiDamageInput(
    value: ReadOnlySignal<Vec<Damage>>,
    onchange: Callback<Vec<Damage>>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 items-end",
            div { class: "grid grid-cols-[1fr_1fr_auto_1fr_1fr_auto] gap-2 w-full",
                for (idx , dmg) in value().iter().enumerate() {
                    DamageInput {
                        grid: true,
                        value: dmg.clone(),
                        onchange: move |d| {
                            let mut dmg = value();
                            dmg[idx] = d;
                            onchange.call(value());
                        },
                    }
                    button {
                        class: "px-4 py-2 text-xl",
                        onclick: move |_| {
                            let mut dmg = value();
                            dmg.remove(idx);
                            onchange.call(value());
                        },
                        "-"
                    }
                }
            }
            button {
                class: "w-fit px-4 py-2 text-xl",
                onclick: move |_| {
                    let mut d = value();
                    d.push(Damage {
                        dice: Dice {
                            count: 1,
                            sides: 4,
                            modifier: None,
                        },
                        damage_type: "".into(),
                    });
                    onchange.call(d);
                },
                "+"
            }
        }
    }
}

#[component]
pub fn SizeSelector(value: ReadOnlySignal<Size>, onchange: Callback<Size>) -> Element {
    rsx! {
        select {
            value: "{value()}",
            onchange: move |e| {
                let size = e.value().into();
                onchange.call(size);
            },
            option { value: "Tiny", "Tiny" }
            option { value: "Small", "Small" }
            option { value: "Medium", "Medium" }
            option { value: "Large", "Large" }
            option { value: "Huge", "Huge" }
            option { value: "Gargantuan", "Gargantuan" }
        }
    }
}

#[component]
pub fn SizeSelectorSignal(size: Signal<Size>) -> Element {
    rsx! {
        SizeSelector { value: size(), onchange: move |s| size.set(s) }
    }
}
