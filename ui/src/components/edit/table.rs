use dioxus::prelude::*;
use types::meta::Table;

use crate::components::view::Pair;

#[component]
pub fn TableEdit(
    table: Table,
    onchange: Option<Callback<Table>>,
    oninput: Option<Callback<Table>>,
) -> Element {
    let onchange = onchange.unwrap_or_default();
    let oninput = oninput.unwrap_or_default();
    let mut table = use_signal(|| table.clone());

    let max_cols = table()
        .rows
        .iter()
        .map(|row| row.columns.len())
        .max()
        .unwrap_or_default();

    let mut table_input = move |row: usize, col: usize, value: String| {
        let mut t = table();
        t.set(row, col, value);
        table.set(t);
        oninput.call(table());
    };

    let table_change = move || {
        onchange.call(table());
    };

    rsx! {
        span { class: "inline-flex flex-wrap justify-between gap-x-2",
            Pair { name: "Name",
                input {
                    r#type: "text",
                    value: "{table().name}",
                    oninput: move |e| {
                        let mut t = table();
                        let name = e.value();
                        t.name = name;
                        table.set(t);
                        oninput.call(table());
                    },
                    onchange: move |_| {
                        table_change();
                    },
                }
            }
            Pair { name: "Columns",
                input {
                    r#type: "number",
                    value: max_cols,
                    onchange: move |e| {
                        let mut t = table();
                        let max_cols = e.value().parse();
                        if let Ok(max_cols) = max_cols {
                            t.set_cols(max_cols);
                            table.set(t);
                            oninput.call(table());
                            onchange.call(table());
                        }
                    },
                }
            }
        }
        table { class: "w-full border",
            thead {
                tr { class: "border-b *:p-1",
                    for i in 0..max_cols {
                        th {
                            input {
                                class: "w-full",
                                r#type: "text",
                                value: table().get(0, i),
                                oninput: move |e| {
                                    table_input(0, i, e.value());
                                },
                                onchange: move |_| {
                                    table_change();
                                },
                            }
                        }
                    }
                }
            }
            tbody {
                for (i , row) in table().rows.iter().skip(1).enumerate() {
                    tr { class: "*:p-1",
                        for (j , col) in row.columns.iter().enumerate() {
                            td {
                                input {
                                    class: "w-full",
                                    r#type: "text",
                                    value: "{col}",
                                    oninput: move |e| {
                                        table_input(i, j, e.value());
                                    },
                                    onchange: move |_| {
                                        table_change();
                                    },
                                }
                            }
                        }
                        td {
                            button {
                                class: "px-2",
                                onclick: move |_| {
                                    let mut t = table();
                                    t.remove_row(i + 1);
                                    table.set(t);
                                    oninput.call(table());
                                    onchange.call(table());
                                },
                                "Remove"
                            }
                        }
                    }
                }
                button {
                    class: "p-2",
                    onclick: move |_| {
                        let mut t = table();
                        t.add_row();
                        table.set(t);
                        oninput.call(table());
                        onchange.call(table());
                    },
                    "Add Row"
                }
            }
        }
    }
}

#[component]
pub fn MultiTableEdit(
    tables: Vec<Table>,
    onchange: Option<Callback<Vec<Table>>>,
    oninput: Option<Callback<Vec<Table>>>,
) -> Element {
    let onchange = onchange.unwrap_or_default();
    let oninput = oninput.unwrap_or_default();
    let mut tables = use_signal(|| tables.clone());

    let mut table_input = move |idx: usize, table: Table| {
        let mut t = tables();
        t[idx] = table;
        tables.set(t);
        oninput.call(tables());
    };

    let table_change = move || {
        onchange.call(tables());
    };

    rsx! {
        div {
            for (i , table) in tables().iter().enumerate() {
                div { class: "flex flex-col gap-y-2",
                    TableEdit {
                        table: table.clone(),
                        onchange: move |_| table_change(),
                        oninput: move |t| table_input(i, t),
                    }
                    button {
                        onclick: move |_| {
                            let mut t = tables();
                            t.remove(i);
                            tables.set(t);
                            oninput.call(tables());
                        },
                        "Remove Table"
                    }
                }
            }

            button {
                onclick: move |_| {
                    let mut t = tables();
                    let mut new_table = Table::default();
                    new_table.add_row();
                    new_table.set_cols(1);
                    t.push(new_table);
                    tables.set(t);
                    oninput.call(tables());
                },
                "Add new Table"
            }
        }
    }
}
