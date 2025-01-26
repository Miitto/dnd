use dioxus::prelude::*;
use types::meta::Table as TableT;
use types::stat_block::StatBlock;

mod description;
pub use description::*;

pub mod spell;
pub mod spell_list;

#[component]
pub fn Pair(
    name: String,
    grid: Option<bool>,
    align: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let grid = grid.unwrap_or(false);

    let display = if grid {
        "grid grid-cols-subgrid col-span-2 gap-x-2"
    } else {
        ""
    };

    let align = if align.unwrap_or(false) {
        "items-center"
    } else {
        ""
    };

    let class = class.unwrap_or_default();
    rsx! {
        p { class: "{display} {align} {class}",
            b { class: "mr-2", "{name}:" }
            {children}
        }
    }
}

#[component]
pub fn PairLi(name: String, children: Element) -> Element {
    rsx! {
        li {
            Pair { name, children }
        }
    }
}

#[component]
pub fn Table(table: TableT) -> Element {
    rsx! {
        table { class: "border",
            thead { class: "font-bold",
                if table.show_name {
                    tr { class: "*:px-2 *:py-1 border-b *:text-left",
                        th { colspan: table.rows[0].columns.len(), "{table.name}" }
                    }
                }
                tr { class: "*:px-2 *:py-1 border-b *:text-left",
                    for col in table.rows[0].columns.iter() {
                        th { "{col}" }
                    }
                }
            }
            tbody {
                for (idx , row) in table.rows[1..].iter().enumerate() {
                    tr { class: "*:px-2 *:py-1 even:bg-muted",
                        if table.ordered {
                            td { "{idx + 1}" }
                        }
                        for cell in row.columns.iter() {
                            td { "{cell}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn StatBlockView(stat_block: StatBlock) -> Element {
    rsx! {
        div { class: "border p-4 flex flex-col lg:flex-row gap-4",
            div { class: "flex flex-col flex-1 gap-y-2",
                h1 { class: "text-2xl", "{stat_block.name}" }
                em { class: "text-lg",
                    "{stat_block.size} {stat_block.creature_type}"
                    if let Some(align) = stat_block.alignment.as_ref() {
                        ", {align}"
                    }
                }
                hr {}

                Pair { name: "Armor Class", "{stat_block.armor_class}" }
                Pair { name: "Hit Points", "{stat_block.hit_points}" }
                Pair { name: "Speed", "{stat_block.speed}" }

                hr {}

                table {
                    thead {
                        tr {
                            th { "STR" }
                            th { "DEX" }
                            th { "CON" }
                            th { "INT" }
                            th { "WIS" }
                            th { "CHA" }
                        }
                    }
                    tbody {
                        tr { class: "*:text-center",
                            td { "{stat_block.attributes.str_str()}" }
                            td { "{stat_block.attributes.dex_str()}" }
                            td { "{stat_block.attributes.con_str()}" }
                            td { "{stat_block.attributes.int_str()}" }
                            td { "{stat_block.attributes.wis_str()}" }
                            td { "{stat_block.attributes.cha_str()}" }
                        }
                    }
                }
                hr {}

                if !stat_block.saving_throws.is_empty() {
                    Pair { name: "Saving Throws", "{stat_block.saving_throws.join(\",\")}" }
                }
                if !stat_block.damage_vulnerabilities.is_empty() {
                    Pair { name: "Damage Vulnerabilities",
                        "{stat_block.damage_vulnerabilities.join(\", \")}"
                    }
                }
                if !stat_block.damage_resistances.is_empty() {
                    Pair { name: "Damage Resistances", "{stat_block.damage_resistances.join(\", \")}" }
                }
                if !stat_block.damage_immunities.is_empty() {
                    Pair { name: "Damage Immunities", "{stat_block.damage_immunities.join(\", \")}" }
                }
                if !stat_block.condition_immunities.is_empty() {
                    Pair { name: "Condition Immunities",
                        "{stat_block.condition_immunities.join(\", \")}"
                    }
                }
                {
                    let mut list = Vec::new();
                    if let Some(darkvision) = stat_block.darkvision {
                        list.push(format!("Darkvision {darkvision}ft"));
                    }
                    if let Some(perception) = stat_block.passive_perception {
                        list.push(format!("Passive Perception {perception}"));
                    }
                    list.extend(stat_block.senses.iter().cloned());
                    let string = list.join(", ");
                    rsx! {
                        if !string.is_empty() {
                            Pair { name: "Senses", "{string}" }
                        }
                    }
                }
                if !stat_block.languages.is_empty() {
                    Pair { name: "Languages", "{stat_block.languages.join(\", \")}" }
                }
                if stat_block.challenge_rating.is_some() && stat_block.proficiency_bonus.is_some() {
                    span { class: "inline-flex justify-between w-full gap-2",
                        if let Some(challenge) = stat_block.challenge_rating {
                            Pair { name: "Challenge", "{challenge}" }
                        }

                        if let Some(proficiency) = stat_block.proficiency_bonus.as_ref() {
                            Pair { name: "Proficiency Bonus", "{proficiency}" }
                        }
                    }
                }
            }
            div { class: "flex flex-col flex-1 gap-y-2",
                if !stat_block.traits.is_empty() {
                    div {
                        h2 { "Traits" }
                        hr {}
                        div { class: "flex flex-col gap-y-2",
                            for nd in stat_block.traits {
                                Pair { name: nd.name, Description {description: nd.description} }
                            }
                        }
                    }
                }

                if !stat_block.actions.is_empty() {
                    div {
                        h2 { "Actions" }
                        hr {}
                        div { class: "flex flex-col gap-y-2",
                            for nd in stat_block.actions {
                                Pair { name: nd.name, Description{ description: nd.description } }
                            }
                        }
                    }
                }

                if !stat_block.reactions.is_empty() {
                    div {
                        h2 { "Reactions" }
                        hr {}
                        div { class: "flex flex-col gap-y-2",
                            for nd in stat_block.reactions {
                                Pair { name: nd.name, Description {description: nd.description} }
                            }
                        }
                    }
                }
            }
        }
    }
}
