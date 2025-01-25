use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::extensions::IsFalse;
use crate::Named;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize)]
pub struct Table {
    pub name: String,
    #[serde(skip_serializing_if = "bool::is_false")]
    pub show_name: bool,
    pub ordered: bool,
    pub rows: Vec<TableRow>,
}

impl Named for Table {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(into = "Vec<String>", from = "Vec<String>")]
pub struct TableRow {
    pub columns: Vec<String>,
}

impl From<Vec<String>> for TableRow {
    fn from(value: Vec<String>) -> TableRow {
        TableRow { columns: value }
    }
}

impl From<TableRow> for Vec<String> {
    fn from(value: TableRow) -> Vec<String> {
        value.columns
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Table, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        #[allow(non_camel_case_types)]
        enum Field {
            Name,
            Rows,
            Ordered,
            Show_Name,
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
                        Field::Show_Name => {
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

impl Table {
    pub fn set(&mut self, row: usize, col: usize, value: String) {
        if self.rows.len() <= row {
            self.rows.resize(row + 1, TableRow { columns: vec![] });
            self.set_cols(self.col_count());
        }
        if self.rows[row].columns.len() <= col {
            self.rows[row].columns.resize(col + 1, "".to_string());
        }
        self.rows[row].columns[col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&str> {
        self.rows
            .get(row)
            .and_then(|r| r.columns.get(col).map(|s| s.as_str()))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut String> {
        self.rows.get_mut(row).and_then(|r| r.columns.get_mut(col))
    }

    pub fn set_cols(&mut self, col_count: usize) {
        for row in &mut self.rows {
            if row.columns.len() != col_count {
                row.columns.resize(col_count, "".to_string());
            }
        }
    }

    pub fn col_count(&self) -> usize {
        self.rows.iter().map(|r| r.columns.len()).max().unwrap_or(0)
    }

    pub fn add_row(&mut self) {
        let col_count = self.col_count();
        self.rows.push(TableRow {
            columns: vec!["".to_string(); col_count],
        });
    }

    pub fn remove_row(&mut self, row: usize) {
        self.rows.remove(row);
    }
}
