use std::sync::Arc;

use dioxus::prelude::*;

use types::{common::Dice, spells::OnSave, stores::Store};

use crate::components::{
    info::Pair,
    inputs::{AttrDropdown, DiceInput, MultiDamageInput, StringList},
};

use types::spells::Components as ComponentsT;

use types::common::Attribute;

#[component]
pub fn SpellEdit(id: String) -> Element {
    let store = use_context::<Store>();

    let cantrip_path = store
        .get_path()
        .map(|p| p.join(types::fs::constants::SPELL_CANTRIPS_PATH));
    let levelled_path = store
        .get_path()
        .map(|p| p.join(types::fs::constants::SPELL_LEVELS_PATH));

    let all = store.spells;
    let lists = store.spell_lists;

    let all_clone = all.clone();

    let spell = use_memo(move || {
        let spell = all.get(&id).unwrap_or_default();

        if spell.name != id {
            let mut clone = (*spell).clone();
            clone.name = id.clone();
            let arc = Arc::new(clone);
            all.store
                .lock()
                .expect("Failed to lock Spells when inserting new spell")
                .insert(id.clone(), arc.clone());

            let mut lists = lists
                .store
                .lock()
                .expect("Failed to lock SpellLists on new spell");

            *lists = lists
                .iter()
                .map(|list| {
                    let mut list = (**list).clone(); // Clone the list so we can insert the new spell without borrowing issues
                    list.found(arc.clone());
                    Arc::new(list)
                })
                .collect();

            arc
        } else {
            spell
        }
    });

    // region: Signal
    let name = use_signal(move || spell().name.clone());
    let level = use_signal(move || spell().level);
    let school = use_signal(move || spell().school.clone());
    let cast_time = use_signal(move || spell().cast_time.clone());
    let range = use_signal(move || spell().range.clone());
    let duration = use_signal(move || spell().duration.clone());

    let components = use_signal(move || spell().components.clone());

    let save_attr = use_signal(move || spell().save);
    let on_save = use_signal(move || spell().on_save);

    let description = use_signal(move || spell().description.clone());

    let at_higher_levels = use_signal(move || spell().at_higher_levels.clone());

    let mut ritual = use_signal(move || spell().ritual);
    let mut concentration = use_signal(move || spell().concentration);

    let mut damages = use_signal(move || spell().damage.clone());
    let mut heal = use_signal(move || spell().heal);

    let serialized = use_memo(move || {
        let mut s = (*spell()).clone();
        s.level = level();
        s.school = school();
        s.cast_time = cast_time();
        s.range = range();
        s.duration = duration();
        s.components = components();
        s.save = save_attr();
        s.on_save = on_save();
        s.description = description();
        s.at_higher_levels = at_higher_levels();
        s.ritual = ritual();
        s.concentration = concentration();
        s.damage = damages();
        s.heal = heal();

        all_clone.set(s.name.clone(), s.clone());

        s.serialize_pretty().unwrap_or_default()
    });
    // endregion

    rsx! {
        CoreBlock {
            name,
            level,
            school,
            cast_time,
            range,
            duration,
        }
        hr {}
        div { class: "flex flex-col",
            Checkbox { name: "Concentration",
                input {
                    r#type: "checkbox",
                    checked: concentration,
                    onchange: move |e| {
                        let checked = e.checked();
                        concentration.set(checked);
                    },
                }
            }

            if level() > 0 {
                Checkbox { name: "Ritual",
                    input {
                        r#type: "checkbox",
                        checked: ritual,
                        onchange: move |e| {
                            let checked = e.checked();
                            ritual.set(checked);
                        },
                    }
                }
            }
        }
        hr {}

        ComponentBlock { components }
        hr {}

        SaveBlock { attr: save_attr, on_save }
        hr {}
        TextAreas { description, at_higher_levels }
        hr {}
        div { class: "flex flex-col",
            h2 { "Base" }
            h3 { "Damage" }
            div {
                MultiDamageInput {
                    value: damages(),
                    onchange: move |d| {
                        damages.set(d);
                    },
                }
            }

            h3 { "Healing" }
            div {
                DiceInput {
                    value: heal().unwrap_or_default(),
                    onchange: move |d: Dice| {
                        if d.is_effective_zero() {
                            heal.set(None);
                        } else {
                            heal.set(Some(d));
                        }
                    },
                }
            }
        }

        hr {}
        textarea {
            class: "w-full resize-none h-fit max-h-[50svh] min-h-40",
            value: "{serialized()}",
            readonly: true,
        }
        br {}
        button {
            class: "px-4 py-2 rounded border w-fit h-fit",
            onclick: move |_| {
                let path = if level() == 0 {
                    cantrip_path.clone()
                } else {
                    levelled_path.clone().map(|p| p.join(format!("{}", level())))
                };
                let named = path
                    .map(|p| {
                        p.join(
                            format!(
                                "{}.json",
                                spell().name.to_lowercase().replace(" ", "_").replace("-", "_"),
                            ),
                        )
                    });
                if let Some(p) = named {
                    let dir = p.parent().expect("Failed to get parent directory");
                    std::fs::create_dir_all(dir).expect("Failed to create directory");
                    std::fs::write(p, serialized()).expect("Failed to write spell to file");
                }
            },
            "Save"
        }
    }
}

#[component]
fn Checkbox(name: String, children: Element) -> Element {
    rsx! {
        fieldset { class: "inline-flex gap-2 items-center",
            {children}
            label { "{name}" }
        }
    }
}

#[component]
fn CoreBlock(
    name: Signal<String>,
    level: Signal<u8>,
    school: Signal<String>,
    cast_time: Signal<String>,
    range: Signal<String>,
    duration: Signal<String>,
) -> Element {
    rsx! {
        div { class: "grid grid-cols-auto-fr space-y-4",
            Pair { name: "Name", grid: true,
                input { value: "{name}", oninput: move |e| name.set(e.value()) }
            }

            Pair { name: "Level", grid: true,
                input {
                    r#type: "number",
                    min: 0,
                    max: 9,
                    value: level,
                    oninput: move |e| level.set(e.value().parse().unwrap_or_default()),
                }
            }

            Pair { name: "School", grid: true,
                select {
                    value: "{school}",
                    onchange: move |e| school.set(e.value()),
                    option { value: "Abjuration", "Abjuration" }
                    option { value: "Conjuration", "Conjuration" }
                    option { value: "Divination", "Divination" }
                    option { value: "Enchantment", "Enchantment" }
                    option { value: "Evocation", "Evocation" }
                    option { value: "Illusion", "Illusion" }
                    option { value: "Necromancy", "Necromancy" }
                    option { value: "Transmutation", "Transmutation" }
                }
            }

            Pair { name: "Cast Time", grid: true,
                input {
                    value: "{cast_time}",
                    oninput: move |e| cast_time.set(e.value()),
                }
            }

            Pair { name: "Range", grid: true,
                input {
                    value: "{range}",
                    oninput: move |e| {
                        let val = e.value().trim().to_string();
                        let without_feet = val.ends_with(" feet");
                        let r = if without_feet {
                            let v = val.trim_end_matches(" feet").to_string();
                            if v.parse::<u32>().is_ok() { v } else { val }
                        } else {
                            val
                        };
                        range.set(r);
                    },
                }
            }

            Pair { name: "Duration", grid: true,
                input {
                    value: "{duration}",
                    oninput: move |e| duration.set(e.value()),
                }
            }
        }
    }
}

#[component]
fn ComponentBlock(components: Signal<ComponentsT>) -> Element {
    rsx! {
        h2 { "Components" }
        div { class: "flex flex-col",
            Checkbox { name: "Verbal",
                input {
                    r#type: "checkbox",
                    checked: components().verbal,
                    onchange: move |e| {
                        let checked = e.checked();
                        let mut comp = components();
                        comp.verbal = checked;
                        components.set(comp);
                    },
                }
            }

            Checkbox { name: "Somatic",
                input {
                    r#type: "checkbox",
                    checked: components().somatic,
                    onchange: move |e| {
                        let checked = e.checked();
                        let mut comp = components();
                        comp.somatic = checked;
                        components.set(comp);
                    },
                }
            }

            h3 { class: "mt-2", "Materials" }
            div { class: "max-h-[50svh] overflow-y-auto",
                StringList {
                    name: "New",
                    list: components().material,
                    oninput: move |changed| {
                        let mut comp = components();
                        comp.material = changed;
                        components.set(comp);
                    },
                }
            }
        }
    }
}

#[component]
fn SaveBlock(attr: Signal<Option<Attribute>>, on_save: Signal<Option<OnSave>>) -> Element {
    let attr_string =
        use_memo(move || attr().map(|a| a.as_short().to_string()).unwrap_or_default());
    rsx! {
        h2 { "Save" }
        div { class: "grid grid-cols-auto-2 space-y-4",
            Pair { name: "Attribute", grid: true,
                AttrDropdown {
                    value: "{attr_string}",
                    allow_none: true,
                    onchange: move |a: String| {
                        if a.is_empty() {
                            attr.set(None);
                        } else {
                            attr.set(
                                Some(
                                    std::convert::TryInto::<Attribute>::try_into(a.as_str())
                                        .expect("Dropdown gave invalid attribute"),
                                ),
                            );
                        }
                    },
                }
            }
            if let Some(_) = attr() {
                Pair { name: "On Save", grid: true,
                    select {
                        value: "{on_save().unwrap_or_default()}",
                        onchange: move |e| on_save.set(Some(e.value().into())),
                        option { value: "Half", "Half" }
                        option { value: "None", "None" }
                    }
                }
            }
        }
    }
}

#[component]
fn TextAreas(description: Signal<String>, at_higher_levels: Signal<Option<String>>) -> Element {
    rsx! {
        h2 { "Description" }
        textarea {
            class: "w-full resize-none h-fit max-h-[50svh] min-h-40",
            value: description(),
            oninput: move |e| description.set(e.value()),
        }
        h2 { "At Higher Levels" }
        textarea {
            class: "w-full resize-none h-fit max-h-[50svh] min-h-32",
            value: "{at_higher_levels().unwrap_or_default()}",
            oninput: move |e| {
                let val = e.value();
                if val.is_empty() {
                    at_higher_levels.set(None);
                } else {
                    at_higher_levels.set(Some(val));
                }
            },
        }
    }
}
