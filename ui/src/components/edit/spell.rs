use std::sync::{Arc, Mutex};

use dioxus::{logger::tracing, prelude::*};

use types::{
    extensions::ForceLock,
    mechanics::Dice,
    meta::{Description, Source},
    spells::{OnSave, Spell},
    stores::Store,
};

use crate::components::{
    edit::{
        AttrDropdown, Checkbox, DescriptionInputSignal, DiceInput, MultiDamageInput,
        SourceInputSignal, StringList, TextAreaSignal,
    },
    view::Pair,
};

use types::spells::Components as ComponentsT;

use types::mechanics::Attribute;

use types::stores::Saveable;

#[derive(Props, Clone)]
pub struct SpellEditProps {
    spell: Arc<Mutex<Spell>>,
}

impl PartialEq for SpellEditProps {
    fn eq(&self, other: &Self) -> bool {
        let self_lock = self.spell.force_lock();
        let other_lock = other.spell.force_lock();

        *self_lock == *other_lock
    }
}

pub fn SpellEdit(props: SpellEditProps) -> Element {
    let all = use_context::<Store>().spells;
    let spell_locked = props.spell;
    let spell = spell_locked.force_lock();

    // region: Signal
    let name = use_signal(|| spell.name.clone());
    let source = use_signal(|| spell.source.clone());
    let level = use_signal(|| spell.level);
    let school = use_signal(|| spell.school.clone());
    let cast_time = use_signal(|| spell.cast_time.clone());
    let range = use_signal(|| spell.range.clone());
    let duration = use_signal(|| spell.duration.clone());

    let components = use_signal(|| spell.components.clone());

    let save_attr = use_signal(|| spell.save);
    let on_save = use_signal(|| spell.on_save);

    let description = use_signal(|| spell.description.clone());

    let at_higher_levels = use_signal(|| spell.at_higher_levels.clone().unwrap_or_default());

    let mut ritual = use_signal(|| spell.ritual);
    let mut concentration = use_signal(|| spell.concentration);

    let mut damages = use_signal(|| spell.damage.clone());
    let mut heal = use_signal(|| spell.heal);

    drop(spell);

    let serialized = use_memo(move || {
        let mut spell = spell_locked.force_lock();
        spell.name = name();
        spell.source = source();
        spell.level = level();
        spell.school = school();
        spell.cast_time = cast_time();
        spell.range = range();
        spell.duration = duration();
        spell.components = components();
        spell.save = save_attr();
        spell.on_save = on_save();
        spell.description = description();
        spell.at_higher_levels = if at_higher_levels().is_empty() {
            None
        } else {
            Some(at_higher_levels())
        };
        spell.ritual = ritual();
        spell.concentration = concentration();
        spell.damage = damages();
        spell.heal = heal();

        spell.serialize_pretty().unwrap_or_default()
    });
    // endregion

    rsx! {
        CoreBlock {
            name,
            source,
            level,
            school,
            cast_time,
            range,
            duration,
        }
        hr {}
        div { class: "flex flex-col",
            Checkbox {
                name: "Concentration",
                checked: concentration(),
                onchange: move |checked| {
                    concentration.set(checked);
                },
            }

            if level() > 0 {
                Checkbox {
                    name: "Ritual",
                    checked: ritual(),
                    onchange: move |checked| {
                        ritual.set(checked);
                    },
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
                all.save(name().as_str()).unwrap_or_else(|e| tracing::error!("{}", e));
            },
            "Save"
        }
    }
}

#[component]
fn CoreBlock(
    name: Signal<String>,
    source: Signal<Source>,
    level: Signal<u8>,
    school: Signal<String>,
    cast_time: Signal<String>,
    range: Signal<String>,
    duration: Signal<String>,
) -> Element {
    rsx! {
        div { class: "grid grid-cols-auto-fr space-y-4",
            div { class: "grid grid-cols-subgrid col-span-2 gap-y-2 w-full",
                Pair {
                    name: "Name",
                    grid: true,
                    align: true,
                    class: "inline-flex items-center gap-2",
                    input {
                        class: "flex-grow",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                }
                Pair { name: "Source", align: true, grid: true,
                    SourceInputSignal { source }
                }
            }

            Pair { name: "Level", grid: true, align: true,
                input {
                    r#type: "number",
                    min: 0,
                    max: 9,
                    value: level,
                    oninput: move |e| level.set(e.value().parse().unwrap_or_default()),
                }
            }

            Pair { name: "School", grid: true, align: true,
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

            Pair { name: "Cast Time", grid: true, align: true,
                input {
                    value: "{cast_time}",
                    oninput: move |e| cast_time.set(e.value()),
                }
            }

            Pair { name: "Range", grid: true, align: true,
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

            Pair { name: "Duration", grid: true, align: true,
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
            Checkbox {
                name: "Verbal",
                checked: components().verbal,
                onchange: move |checked| {
                    let mut comp = components();
                    comp.verbal = checked;
                    components.set(comp);
                },
            }

            Checkbox {
                name: "Somatic",
                checked: components().somatic,
                onchange: move |checked| {
                    let mut comp = components();
                    comp.somatic = checked;
                    components.set(comp);
                },
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
            Pair { name: "Attribute", grid: true, align: true,
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
                Pair { name: "On Save", grid: true, align: true,
                    select {
                        value: "{on_save().unwrap_or_default()}",
                        onchange: move |e| on_save.set(Some(e.value().into())),
                        option { value: "Half", "Half" }
                        option { value: "Debuff", "Debuff" }
                        option { value: "None", "None" }
                    }
                }
            }
        }
    }
}

#[component]
fn TextAreas(description: Signal<Description>, at_higher_levels: Signal<String>) -> Element {
    rsx! {
        h2 { "Description" }
        DescriptionInputSignal { description }
        h2 { "At Higher Levels" }
        TextAreaSignal { value: at_higher_levels }
    }
}
