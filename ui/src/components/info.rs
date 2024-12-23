use dioxus::prelude::*;
use types::common::Table as TableT;

#[component]
pub fn Pair(name: String, children: Element) -> Element {
    rsx! {
        p { class: "inline-flex gap-x-2",
            b { "{name}:" }
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

#[derive(Debug, Clone, PartialEq)]
enum Line {
    Text(String),
    List(Vec<String>),
}

#[component]
#[allow(clippy::manual_strip)]
pub fn Description(description: String) -> Element {
    let lines = use_memo(move || {
        let lines = description
            .lines()
            .map(|el| el.to_string())
            .collect::<Vec<String>>();

        let mut result = vec![];

        let mut in_list = false;

        let mut list = vec![];

        for line in lines {
            if in_list {
                if line.starts_with(".") {
                    list.push(line[1..].to_string());
                    continue;
                } else {
                    result.push(Line::List(list));
                    list = vec![];
                    in_list = false;
                }
            } else if line.ends_with(":") {
                in_list = true;
            }
            result.push(Line::Text(line));
        }

        if !list.is_empty() {
            result.push(Line::List(list));
        }

        result
    });
    rsx! {
        for line in lines().iter() {
            match line {
                Line::Text(text) => rsx! {
                    p { class: "py-2", "{text}" }
                },
                Line::List(list) => rsx! {
                    ul { class: "list-disc pl-6",
                        for item in list.iter() {
                            li { "{item}" }
                        }
                    }
                },
            }
        }
    }
}
