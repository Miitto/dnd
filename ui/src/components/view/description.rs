use dioxus::prelude::*;
use types::extensions::ForceLock;
use types::meta::{
    description::Node, Description as DescriptionT, DescriptionEmbed, DescriptionLine, Link,
};

use crate::components::view::{StatBlockView, Table};

#[component]
#[allow(clippy::manual_strip)]
pub fn Description(description: DescriptionT) -> Element {
    rsx! {
        for line in description.lines {
            LineView { line, in_list: false }
        }
    }
}

#[component]
fn LineView(line: DescriptionLine, in_list: bool) -> Element {
    match line {
        DescriptionLine::Text(node) => {
            rsx! {
                NodeRender { node }
            }
        }
        DescriptionLine::Embed(embed) => match *embed {
            DescriptionEmbed::StatBlock(link) => match *link {
                Link::Found(stat_block) => {
                    rsx! {
                        StatBlockView { stat_block: stat_block.force_lock().clone() }
                    }
                }
                Link::NotFound(name) => {
                    rsx! {
                        p {
                            "Stat Block "
                            code { "{name}" }
                            " not found"
                        }
                    }
                }
            },
            DescriptionEmbed::Table(link) => match link {
                Link::Found(table) => {
                    rsx! {
                        Table { table: table.force_lock().clone() }
                    }
                }
                Link::NotFound(name) => {
                    rsx! {
                        p {
                            "Table "
                            code { "{name}" }
                            " not found"
                        }
                    }
                }
            },
        },
    }
}

#[component]
fn NodeRender(node: Node) -> Element {
    match node {
        Node::Root(root) => {
            rsx! {
                for node in root.children {
                    NodeRender { node }
                }
            }
        }
        Node::Text(text) => {
            rsx! {
                {text.value}
            }
        }
        Node::Emphasis(emphasis) => {
            rsx! {
                em {
                    for node in emphasis.children {
                        NodeRender { node }
                    }
                }
            }
        }
        Node::Strong(strong) => {
            rsx! {
                strong {
                    for node in strong.children {
                        NodeRender { node }
                    }
                }
            }
        }
        Node::Paragraph(paragraph) => {
            rsx! {
                p {
                    class: "my-1",
                    for node in paragraph.children {
                        NodeRender { node }
                    }
                }
            }
        }
        Node::Heading(heading) => match heading.depth {
            1 => {
                rsx! {
                    h1 {
                        for node in heading.children {
                            NodeRender { node }
                        }
                    }
                }
            }
            2 => {
                rsx! {
                    h2 {
                        for node in heading.children {
                            NodeRender { node }
                        }
                    }
                }
            }
            3 => {
                rsx! {
                    h3 {
                        for node in heading.children {
                            NodeRender { node }
                        }
                    }
                }
            }
            4 => {
                rsx! {
                    h4 {
                        for node in heading.children {
                            NodeRender { node }
                        }
                    }
                }
            }
            5 => {
                rsx! {
                    h5 {
                        for node in heading.children {
                            NodeRender { node }
                        }
                    }
                }
            }
            6 => {
                rsx! {
                    h6 {
                        for node in heading.children {
                            NodeRender { node }
                        }
                    }
                }
            }
            _ => unreachable!(),
        },
        Node::List(list) => match list.ordered {
            true => {
                rsx! {
                    ol { class: "list-decimal pl-6",
                        for node in list.children {
                            NodeRender { node }
                        }
                    }
                }
            }
            false => {
                rsx! {
                    ul { class: "list-disc pl-6",
                        for node in list.children {
                            NodeRender { node }
                        }
                    }
                }
            }
        },
        Node::ListItem(list_item) => {
            rsx! {
                li {
                    for node in list_item.children {
                        NodeRender { node }
                    }
                }
            }
        }
        Node::Break(_) => {
            rsx! {
                br {}
            }
        }
        Node::InlineCode(inline_code) => {
            rsx! {
                InlineCode { text: inline_code.value }
            }
        }
        Node::Code(code_block) => {
            rsx! {
                pre {
                    code { {code_block.value} }
                }
            }
        }
        Node::Blockquote(blockquote) => {
            rsx! {
                blockquote {
                    for node in blockquote.children {
                        NodeRender { node }
                    }
                }
            }
        }
        Node::Delete(delete) => {
            rsx! {
                del {
                    for node in delete.children {
                        NodeRender { node }
                    }
                }
            }
        }
        Node::Link(link) => {
            rsx! {
                Link {
                    to: link.url,
                    class: "underline",
                    for node in link.children {
                        NodeRender { node }
                    }
                }
            }
        }
        _ => {
            eprintln!("Unhandled node: {:?}", node);
            rsx! {
                p { "Unhandled node" }
            }
        }
    }
}

#[component]
pub fn InlineCode(text: String) -> Element {
    if let Some(text) = text.strip_prefix("hover:") {
        let mut parts = text.split('|');

        let name = parts.next().context("InlineHover has no name")?;
        let hover = parts.next().context("InlineHover has no hover text")?;

        return rsx! {
            span { class:"underline", title: hover, {name} }
        };
    }

    rsx! {
        code { {text} }
    }
}
