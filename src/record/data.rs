use crate::{
    filesystem::{PageBuffer, PAGE_SIZE},
    util::{bitmap::Bitmap, error::Error as DBError},
};

use super::{
    column::{DataType, TableMeta},
    table::Pos,
};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum DataValue {
    Int(i32),
    Float(f32),
    VarStr(String),
    Null,
}

impl Default for DataValue {
    fn default() -> Self {
        DataValue::Null
    }
}

/// one row of data
#[derive(Clone)]
pub struct DataEntry {
    pub id: Pos, // it is a auto-generated entry
    pub data: Vec<DataValue>,
}

impl PageBuffer {
    /// Pay attention that no info can be inferred to calculate id,
    /// so it will be zero
    pub fn parse_data_entry(&self, pos: u32, format: &TableMeta, id: &Pos) -> Option<DataEntry> {
        let size = format.entry_size_fixed();
        if pos >= PAGE_SIZE as u32 || pos + size >= PAGE_SIZE as u32 {
            return None;
        }
        // Bitmap
        let len = format.field.len() as u32;
        let bitmap = self.parse_bitmap(pos, len)?;
        let mut cur = pos + (len + 7) / 8;
        let mut values = vec![];
        for i in 0..len {
            let item = &format.field[i as usize];
            let (mut val, nxt) = match item.data_type {
                DataType::Int => (DataValue::Int(self.parse_i32(cur)?), cur + 4),
                DataType::Float => (DataValue::Float(self.parse_f32(cur)?), cur + 4),
                DataType::VarStr(x) => (DataValue::VarStr(self.parse_str_fixed(cur, x)?), cur + x),
            };
            cur = nxt;
            // null value
            if bitmap.get(i).unwrap() {
                val = DataValue::Null;
            }
            values.push(val);
        }
        Some(DataEntry {
            id: id.clone(),
            data: values,
        })
    }

    /// all errors are about data inconsistency, so no need to report
    /// unwrap() is ok.
    pub fn write_data_entry(
        &mut self,
        pos: u32,
        format: &TableMeta,
        data: &DataEntry,
    ) -> Result<(), DBError> {
        let size = format.entry_size_fixed();
        if pos >= PAGE_SIZE as u32 || pos + size >= PAGE_SIZE as u32 {
            return Err(DBError::new("Out of bounds!"));
        }
        let len = format.field.len() as u32;
        let mut bitmap = Bitmap::new(len);
        let mut cur = pos + (len + 7) / 8;
        for i in 0..len {
            cur = match (&format.field[i as usize].data_type, &data.data[i as usize]) {
                (DataType::Int, DataValue::Int(x)) => {
                    self.write_i32(cur, *x);
                    cur + 4
                }
                (DataType::Int, DataValue::Null) => {
                    self.write_i32(cur, 0);
                    bitmap.set(i, true);
                    cur + 4
                }
                (DataType::Float, DataValue::Float(x)) => {
                    self.write_f32(cur, *x);
                    cur + 4
                }
                (DataType::Float, DataValue::Null) => {
                    self.write_f32(cur, 0.0);
                    bitmap.set(i, true);
                    cur + 4
                }
                (DataType::VarStr(len), DataValue::VarStr(x)) => {
                    self.write_str_fixed(cur, x, *len);
                    cur + len
                }
                (DataType::VarStr(len), DataValue::Null) => {
                    // We must write a valid String
                    self.write_str_fixed(cur, &String::from(""), *len);
                    bitmap.set(i, true);
                    cur + len
                }
                _ => {
                    return Err(DBError::new(&format!(
                        "Inconsistent data in column {}",
                        format.field[i as usize].name
                    )));
                }
            }
        }
        self.write_bitmap(pos, &bitmap);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        filesystem::PageBuffer,
        record::{
            column::{DataColumn, DataType, TableMeta},
            data::{DataEntry, DataValue},
            table::Pos,
        },
    };

    fn test_case(entry: &DataEntry, column: &TableMeta) {
        let mut page = PageBuffer::new(0);

        page.write_data_entry(0, column, entry).unwrap();
        let len = column.entry_size_fixed();
        for i in len..len + 64 {
            page.data[i as usize] ^= 0xff;
        }

        assert_eq!(
            page.parse_data_entry(0, column, &Pos(0, 0)).unwrap().data,
            entry.data
        );
    }

    #[test]
    fn check_data_io() {
        fn get_col(n: String, x: DataType) -> DataColumn {
            DataColumn {
                name: n,
                data_type: x,
                is_primary: false,
                is_nullable: true,
                default_value: DataValue::Null,
            }
        }

        let column = TableMeta {
            id: 0,
            field: vec![
                get_col(String::from("name"), DataType::VarStr(35)),
                get_col(String::from("id"), DataType::Int),
                get_col(String::from("height"), DataType::Float),
            ],
            foreign_key: vec![],
            index: vec![],
        };
        test_case(
            &DataEntry {
                id: Pos(0, 0),
                data: vec![
                    DataValue::VarStr(String::from("Alice")),
                    DataValue::Int(1),
                    DataValue::Float(0.5),
                ],
            },
            &column,
        );

        test_case(
            &DataEntry {
                id: Pos(0, 0),
                data: vec![DataValue::Null, DataValue::Null, DataValue::Null],
            },
            &column,
        );

        test_case(
            &DataEntry {
                id: Pos(0, 0),
                data: vec![
                    DataValue::VarStr(String::from("Alice")),
                    DataValue::Null,
                    DataValue::Float(2.33),
                ],
            },
            &column,
        );
    }
}
