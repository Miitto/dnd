use std::fmt;

use markdown::ParseOptions;
use serde::de;
use serde::{Deserialize, Serialize};

use super::Table;
use crate::extensions::AstToString;
use crate::meta::Link;
use crate::stat_block::StatBlock;

pub use markdown::mdast::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum DescriptionLine {
    Text(Node),
    Embed(Box<DescriptionEmbed>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DescriptionEmbed {
    StatBlock(Box<Link<StatBlock>>),
    Table(Link<Table>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Description {
    pub lines: Vec<DescriptionLine>,
}

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        for line in &self.lines {
            match line {
                DescriptionLine::Text(node) => write!(f, "{}", node.ast_to_string())?,
                DescriptionLine::Embed(embed) => match &**embed {
                    DescriptionEmbed::StatBlock(link) => {
                        write!(f, "{{stat_block:{}}}", link.name())?
                    }
                    DescriptionEmbed::Table(link) => write!(f, "{{table:{}}}", link.name())?,
                },
            }
        }

        Ok(())
    }
}

impl From<&str> for Description {
    fn from(value: &str) -> Self {
        let lines = value.lines();

        let lines = lines
            .map(|line| {
                if line.starts_with("{") && line.ends_with("}") {
                    let line = line.trim_start_matches("{").trim_end_matches("}");

                    if let Some((kind, name)) = line.split_once(":") {
                        match kind {
                            "stat_block" => {
                                return DescriptionLine::Embed(Box::new(
                                    DescriptionEmbed::StatBlock(Box::new(Link::NotFound(
                                        name.to_string(),
                                    ))),
                                ));
                            }
                            "table" => {
                                return DescriptionLine::Embed(Box::new(DescriptionEmbed::Table(
                                    Link::NotFound(name.to_string()),
                                )));
                            }
                            _ => {}
                        }
                    }
                }

                DescriptionLine::Text(markdown::to_mdast(line, &ParseOptions::default()).unwrap())
            })
            .collect();

        Description { lines }
    }
}

impl From<String> for Description {
    fn from(value: String) -> Self {
        Description::from(value.as_str())
    }
}

impl<'de> Deserialize<'de> for Description {
    fn deserialize<D>(deserializer: D) -> Result<Description, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct DescriptionVisitor;

        impl de::Visitor<'_> for DescriptionVisitor {
            type Value = Description;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a description")
            }

            fn visit_str<E>(self, value: &str) -> Result<Description, E>
            where
                E: de::Error,
            {
                Ok(value.into())
            }
        }

        deserializer.deserialize_str(DescriptionVisitor)
    }
}

impl Serialize for Description {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = self.to_string();

        serializer.serialize_str(&string)
    }
}
