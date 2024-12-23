use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub ordered: bool,
    pub rows: Vec<TableRow>,
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
            Rows,
            Ordered,
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

                while let Some(row) = seq.next_element()? {
                    rows.push(row);
                }

                Ok(Table {
                    ordered: false,
                    rows,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Table, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut ordered = false;
                let mut rows = None;

                while let Some(key) = map.next_key()? {
                    match key {
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
                    }
                }

                let rows = rows.ok_or_else(|| de::Error::missing_field("rows"))?;
                Ok(Table { ordered, rows })
            }
        }

        const FIELDS: &[&str] = &["rows", "ordered"];
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
