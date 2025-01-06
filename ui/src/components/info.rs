use dioxus::prelude::*;
use types::common::Table as TableT;
use types::stat_block::StatBlock;

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

#[derive(Debug, Clone, PartialEq)]
enum Line {
    Text(String),
    List(Vec<Line>),
}

#[component]
#[allow(clippy::manual_strip)]
pub fn Description(description: String) -> Element {
    let lines = use_memo(move || {
        let lines = description
            .lines()
            .map(|el| el.to_string())
            .collect::<Vec<String>>();

        let mut list = vec![];

        fn parse_depth<I>(iter: &mut I, list: &mut Vec<Line>, depth: usize) -> Option<String>
        where
            I: Iterator<Item = String>,
        {
            while let Some(line) = iter.next() {
                if line.chars().all(char::is_whitespace) {
                    continue;
                }

                let line = line.trim().to_string();

                let line_depth = line.chars().take_while(|c| c == &'.').count();

                let line = line[line_depth..].trim().to_string();

                if line_depth < depth {
                    return Some(line);
                }

                if line_depth > depth {
                    let mut sublist = vec![];
                    sublist.push(Line::Text(line));
                    let text = parse_depth(iter, &mut sublist, line_depth);
                    list.push(Line::List(sublist));
                    if let Some(text) = text {
                        list.push(Line::Text(text));
                    }
                } else {
                    list.push(Line::Text(line));
                }
            }

            None
        }

        let mut iter = lines.into_iter();

        parse_depth(&mut iter, &mut list, 0);

        list
    });
    rsx! {
        for line in lines() {
            LineView { line, in_list: false }
        }
    }
}

#[component]
fn LineView(line: Line, in_list: bool) -> Element {
    match line {
        Line::Text(text) => rsx! {
            if in_list {
                li { class: "py-2", "{text}" }
            } else {
                p { class: "py-2", "{text}" }
            }
        },
        Line::List(list) => rsx! {
            ul { class: "list-disc pl-6",
                for line in list {
                    LineView { line, in_list: true }
                }
            }
        },
    }
}

#[component]
pub fn StatBlockView(stat_block: StatBlock) -> Element {
    rsx! {}
}
