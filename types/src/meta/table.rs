use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::Named;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub name: String,
    pub show_name: bool,
    pub ordered: bool,
    pub rows: Vec<TableRow>,
}

impl Named for Table {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    pub columns: Vec<String>,
}

impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.rows.len()))?;
        for row in &self.rows {
            seq.serialize_element(row)?;
        }
        seq.end()
    }
}

impl Serialize for TableRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.columns.len()))?;
        for column in &self.columns {
            seq.serialize_element(column)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Table, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Rows,
            Ordered,
            ShowName,
        }

        struct TableVisitor;

        impl<'de> Visitor<'de> for TableVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Table or list of TableRow")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Table, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut rows = Vec::new();

                let name: String = seq.next_element()?.ok_or(de::Error::invalid_length(
                    0,
                    &"Table needs at least one element to be the name",
                ))?;

                while let Some(row) = seq.next_element()? {
                    rows.push(row);
                }

                Ok(Table {
                    name,
                    ordered: false,
                    show_name: false,
                    rows,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Table, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut ordered = false;
                let mut rows = None;
                let mut show_name = false;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Rows => {
                            if rows.is_some() {
                                return Err(de::Error::duplicate_field("rows"));
                            }
                            rows = Some(map.next_value()?);
                        }
                        Field::Ordered => {
                            if ordered {
                                return Err(de::Error::duplicate_field("ordered"));
                            }
                            ordered = map.next_value()?;
                        }
                        Field::ShowName => {
                            if ordered {
                                return Err(de::Error::duplicate_field("show_name"));
                            }
                            show_name = map.next_value()?;
                        }
                    }
                }

                let rows = rows.ok_or_else(|| de::Error::missing_field("rows"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                Ok(Table {
                    name,
                    ordered,
                    rows,
                    show_name,
                })
            }
        }

        const FIELDS: &[&str] = &["name", "rows", "ordered", "show_name"];
        deserializer.deserialize_struct("Table", FIELDS, TableVisitor)
    }
}

impl<'a> Deserialize<'a> for TableRow {
    fn deserialize<D>(deserializer: D) -> Result<TableRow, D::Error>
    where
        D: Deserializer<'a>,
    {
        let columns = Vec::<String>::deserialize(deserializer)?;
        Ok(TableRow { columns })
    }
}
