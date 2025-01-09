use dioxus::prelude::*;
use types::meta::Table as TableT;
use types::stat_block::StatBlock;

mod description;
pub use description::*;

pub mod spell;
pub mod spell_list;

#[component]
pub fn Pair(name: String, grid: Option<bool>, class: Option<String>, children: Element) -> Element {
    let grid = grid.unwrap_or(false);

    let display = if grid {
        "grid grid-cols-subgrid col-span-2"
    } else {
        "flex flex-row"
    };

    let class = class.unwrap_or_default();
    rsx! {
        p { class: "{display} gap-x-2 items-center {class}",
            b { class: "h-fit max-w-fit", "{name}:" }
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
    rsx! {}
}
