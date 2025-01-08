use dioxus::prelude::*;
use types::meta::{
    description::Node, Description as DescriptionT, DescriptionEmbed, DescriptionLine,
};

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
            DescriptionEmbed::StatBlock(_) => {
                todo!("StatBlock")
            }
            DescriptionEmbed::Table(_) => {
                todo!("Table")
            }
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
                code { {inline_code.value} }
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
        _ => {
            eprintln!("Unhandled node: {:?}", node);
            rsx! {
                p { "Unhandled node" }
            }
        }
    }
}