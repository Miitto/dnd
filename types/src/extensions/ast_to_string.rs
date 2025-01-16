pub trait AstToString {
    fn ast_to_string(&self) -> String;
}

use markdown::mdast::Node;

impl AstToString for Node {
    fn ast_to_string(&self) -> String {
        match self {
            Node::Root(root) => {
                let mut string = String::new();
                for child in &root.children {
                    string.push_str(&child.ast_to_string());
                }
                string
            }
            Node::Paragraph(paragraph) => {
                let mut string = String::new();
                for child in &paragraph.children {
                    string.push_str(&child.ast_to_string());
                }
                string.push('\n');
                string
            }
            Node::Text(text) => text.value.to_owned(),
            Node::Emphasis(emphasis) => {
                let mut string = String::new();
                string.push('*');
                for child in &emphasis.children {
                    string.push_str(&child.ast_to_string());
                }
                string.push('*');
                string
            }
            Node::Strong(strong) => {
                let mut string = String::new();
                string.push_str("**");
                for child in &strong.children {
                    string.push_str(&child.ast_to_string());
                }
                string.push_str("**");
                string
            }
            Node::Blockquote(blockquote) => {
                let mut string = String::new();
                string.push_str("> ");
                for child in &blockquote.children {
                    string.push_str(&child.ast_to_string());
                }
                string
            }
            Node::InlineCode(code) => {
                let mut string = String::new();
                string.push('`');
                string.push_str(&code.value);
                string.push('`');
                string
            }
            Node::Link(link) => {
                let mut string = String::new();
                string.push('[');
                string.push_str(link.url.as_str());
                string.push_str("](");
                for child in &link.children {
                    string.push_str(&child.ast_to_string());
                }
                string.push(')');
                string
            }
            Node::Break(_) => String::from("\n"),
            _ => String::new(),
        }
    }
}
