use crate::filesystem::LRUBufPageManager;
use crate::index::manager::IndexOptions;
use crate::util::error::Error as DBError;

use super::data::{DataEntry, DataValue};
use super::table::{Pos, RecordManager};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone)]
pub enum DataType {
    /// Integer type
    Int,
    /// (single-precision) Float type
    Float,
    /// Variable length string (length)
    VarStr(u32),
}

pub struct DataColumn {
    pub name: String,
    pub data_type: DataType,
    pub is_primary: bool,
    pub is_nullable: bool,
    pub default_value: DataValue,
}

pub struct ForeignKey {
    pub name: String, // empty means no name
    pub from: Vec<String>,
    pub table: String,
    pub to: Vec<String>,
    pub id: u32,
}

pub struct Index {
    pub col: Vec<String>,
    pub id: u32, // for index storage
}

pub struct TableMeta {
    pub id: u32,
    pub field: Vec<DataColumn>,
    pub foreign_key: Vec<ForeignKey>,
    pub index: Vec<Index>,
}

impl TableMeta {
    /** For each entry (fixed format), the following format is used
     * + Null bitmap (ceil (n / 8) bytes), where n is the column number,
     *   regardless of nullable of primary status
     * + each entry, where:
     *   - Int/Float: 4 bytes,
     *   - VarStr: n bytes
     * The result is calucated in O(n), so store it when needed.
     */
    pub fn entry_size_fixed(&self) -> u32 {
        let ret = (self.field.len() as u32 + 7) / 8;
        ret + self
            .field
            .iter()
            .map(|x| match x.data_type {
                DataType::Float | DataType::Int => 4,
                DataType::VarStr(x) => x,
            })
            .sum::<u32>()
    }
}

pub struct TableMetaManager {
    path: String,
    buf: Rc<RefCell<LRUBufPageManager>>,
    database: String,
}

impl TableMetaManager {
    pub fn new(
        path: &str,
        buf: &Rc<RefCell<LRUBufPageManager>>,
        database: &str,
    ) -> TableMetaManager {
        TableMetaManager {
            path: String::from(path),
            buf: Rc::clone(buf),
            database: String::from(database),
        }
    }

    pub fn read_table(&self, table_name: &str) -> Result<TableMeta, Box<dyn std::error::Error>> {
        let mut res = TableMeta {
            id: 0,
            field: vec![],
            foreign_key: vec![],
            index: vec![],
        };
        // TODO: check whether the table exists
        self.parse_col(table_name, &mut res)?;
        if res.field.is_empty() {
            return Err(Box::new(DBError::new(&format!(
                "table {} not found!",
                table_name
            ))));
        }
        self.parse_fk(table_name, &mut res)?;
        self.parse_idx(table_name, &mut res)?;
        self.get_default(table_name, &mut res)?;
        Ok(res)
    }

    fn create_manager<'a>(
        &'a self,
        file_name: &str,
        meta: &'a TableMeta,
    ) -> Result<RecordManager<'a>, Box<dyn std::error::Error>> {
        let mut path = PathBuf::from(&self.path);
        path.push(file_name);
        let path = match path.to_str() {
            Some(x) => x,
            None => return Err(Box::new(DBError::new("path error"))),
        };
        RecordManager::new(path, &self.buf, meta)
    }

    fn create_col(
        &self,
        table_name: &str,
        meta: &TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let write_meta = gen_col_schema();
        let mut manager = self.create_manager(TABLE_COL_NAME, &write_meta)?;
        if manager.iter().any(|x| {
            x.data[0] == DataValue::VarStr(self.database.clone())
                && x.data[1] == DataValue::VarStr(table_name.to_string())
        }) {
            return Err(Box::new(DBError::new("table already exists!")));
        }

        for (i, item) in meta.field.iter().enumerate() {
            manager.insert(&DataEntry {
                id: Pos(0, 0),
                data: vec![
                    DataValue::VarStr(self.database.clone()),  // db_name
                    DataValue::VarStr(table_name.to_string()), // tb_name
                    DataValue::VarStr(item.name.clone()),      // col_name
                    DataValue::Int(get_property(item)),        // property
                    DataValue::Int(i as i32),                  // order
                ],
            })?;
        }
        Ok(())
    }

    fn parse_col(
        &self,
        table_name: &str,
        meta: &mut TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let write_meta = gen_col_schema();
        let manager = self.create_manager(TABLE_COL_NAME, &write_meta)?;
        let mut res: Vec<(DataColumn, i32)> = vec![];
        manager.iter().for_each(|item| {
            if item.data[0] == DataValue::VarStr(self.database.clone())
                && item.data[1] == DataValue::VarStr(table_name.to_string())
            {
                let property = if let DataValue::Int(x) = item.data[3] {
                    x
                } else {
                    unreachable!()
                };

                res.push((
                    DataColumn {
                        name: if let DataValue::VarStr(x) = &item.data[2] {
                            x.clone()
                        } else {
                            unreachable!();
                        },
                        data_type: match property & 0xffff {
                            0 => DataType::Int,
                            1 => DataType::Float,
                            _ => DataType::VarStr((property & 0xffff) as u32 - 2),
                        },
                        is_primary: property & P_PRIMARY > 0,
                        is_nullable: property & P_NULLABLE > 0,
                        default_value: DataValue::Null,
                    },
                    (if let DataValue::Int(x) = item.data[4] {
                        x
                    } else {
                        unreachable!()
                    }),
                ))
            }
        });
        res.sort_by(|(_, x), (_, y)| x.cmp(y));
        let (x, _): (Vec<DataColumn>, Vec<i32>) = res.into_iter().unzip();
        meta.field = x;
        Ok(())
    }

    fn create_fk(
        &self,
        table_name: &str,
        meta: &TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let write_meta = gen_fk_schema();
        let mut manager = self.create_manager(TABLE_FK_NAME, &write_meta)?;
        for fk_item in &meta.foreign_key {
            for (i, (col1_item, col2_item)) in
                fk_item.from.iter().zip(fk_item.to.iter()).enumerate()
            {
                manager.insert(&DataEntry {
                    id: Pos(0, 0),
                    data: vec![
                        DataValue::VarStr(self.database.clone()),  // db_name
                        DataValue::VarStr(fk_item.name.clone()),   // fk_name
                        DataValue::VarStr(table_name.to_string()), // tb_from_name
                        DataValue::VarStr(fk_item.table.clone()),  // tb_to_name
                        DataValue::VarStr(col1_item.clone()),      // col_from_name,
                        DataValue::VarStr(col2_item.clone()),      // col_to_name
                        DataValue::Int(fk_item.id as i32),         // id
                    ],
                })?;
            }
        }

        Ok(())
    }

    fn parse_fk(
        &self,
        table_name: &str,
        meta: &mut TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let write_meta = gen_fk_schema();
        let manager = self.create_manager(TABLE_FK_NAME, &write_meta)?;
        let mut res: Vec<ForeignKey> = vec![];

        manager.iter().for_each(|item| {
            if item.data[0] == DataValue::VarStr(self.database.clone())
                && item.data[2] == DataValue::VarStr(table_name.to_string())
            {
                let fk_name = if let DataValue::VarStr(x) = &item.data[1] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let f_name = if let DataValue::VarStr(x) = &item.data[4] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let t_table = if let DataValue::VarStr(x) = &item.data[3] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let t_name = if let DataValue::VarStr(x) = &item.data[5] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let id = if let DataValue::Int(x) = &item.data[6] {
                    *x as u32
                } else {
                    unreachable!()
                };
                let mut found = false;
                for res_item in &mut res {
                    if res_item.id == id {
                        res_item.from.push(f_name.clone());
                        res_item.to.push(t_name.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    res.push(ForeignKey {
                        name: fk_name,
                        from: vec![f_name],
                        table: t_table,
                        to: vec![t_name],
                        id,
                    });
                }
            }
        });

        meta.foreign_key = res;
        Ok(())
    }

    pub fn find_fk_constraint(
        &self,
        table_name: &str,
    ) -> Result<Vec<(String, ForeignKey)>, Box<dyn std::error::Error>> {
        let write_meta = gen_fk_schema();
        let manager = self.create_manager(TABLE_FK_NAME, &write_meta)?;
        let mut res: Vec<(String, ForeignKey)> = vec![];

        manager.iter().for_each(|item| {
            if item.data[0] == DataValue::VarStr(self.database.clone())
                && item.data[3] == DataValue::VarStr(table_name.to_string())
            {
                let fk_name = if let DataValue::VarStr(x) = &item.data[1] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let f_name = if let DataValue::VarStr(x) = &item.data[4] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let f_table = if let DataValue::VarStr(x) = &item.data[2] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let t_name = if let DataValue::VarStr(x) = &item.data[5] {
                    x.clone()
                } else {
                    unreachable!()
                };
                let id = if let DataValue::Int(x) = &item.data[6] {
                    *x as u32
                } else {
                    unreachable!()
                };
                let mut found = false;
                for (item_f, res_item) in &mut res {
                    if *item_f == f_table && res_item.id == id {
                        res_item.from.push(f_name.clone());
                        res_item.to.push(t_name.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    res.push((
                        f_table,
                        ForeignKey {
                            name: fk_name,
                            from: vec![f_name],
                            table: table_name.to_string(),
                            to: vec![t_name],
                            id,
                        },
                    ));
                }
            }
        });
        Ok(res)
    }

    fn create_idx(
        &self,
        table_name: &str,
        meta: &TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let write_meta = gen_idx_schema();
        let mut manager = self.create_manager(TABLE_IDX_NAME, &write_meta)?;
        for idx_item in &meta.index {
            for (i, col_item) in idx_item.col.iter().enumerate() {
                manager.insert(&DataEntry {
                    id: Pos(0, 0),
                    data: vec![
                        DataValue::VarStr(self.database.clone()),  // db_name
                        DataValue::VarStr(table_name.to_string()), // tb_name
                        DataValue::Int(idx_item.id as i32),        // idx
                        DataValue::VarStr(col_item.clone()),       // col_name
                        DataValue::Int(i as i32),                  // order
                    ],
                })?;
            }
        }

        Ok(())
    }

    fn parse_idx(
        &self,
        table_name: &str,
        meta: &mut TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let write_meta = gen_idx_schema();
        let manager = self.create_manager(TABLE_IDX_NAME, &write_meta)?;
        let mut res: Vec<(u32, Vec<(String, i32)>)> = vec![];
        manager.iter().for_each(|item| {
            if item.data[0] == DataValue::VarStr(self.database.clone())
                && item.data[1] == DataValue::VarStr(table_name.to_string())
            {
                let idx_id = if let DataValue::Int(x) = &item.data[2] {
                    *x as u32
                } else {
                    unreachable!();
                };
                let col_name = if let DataValue::VarStr(x) = &item.data[3] {
                    x.clone()
                } else {
                    unreachable!();
                };
                let ord = if let DataValue::Int(x) = &item.data[4] {
                    *x
                } else {
                    unreachable!();
                };
                let mut found = false;
                for (x, y) in &mut res {
                    if *x == idx_id {
                        y.push((col_name.clone(), ord));
                        found = true;
                        break;
                    }
                }
                if !found {
                    res.push((idx_id, vec![(col_name, ord)]));
                }
            }
        });

        meta.index = res
            .into_iter()
            .map(|(id, mut col)| Index {
                id,
                col: {
                    col.sort_by_key(|(_, y)| *y);
                    let (x, _): (Vec<String>, Vec<i32>) = col.into_iter().unzip();
                    x
                },
            })
            .collect();

        Ok(())
    }

    fn set_default(&self, name: &str, meta: &TableMeta) -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = self.create_manager(&self.get_path(name), &meta)?;
        manager.set_default_value(&DataEntry {
            id: Pos(0, 0),
            data: meta.field.iter().map(|x| x.default_value.clone()).collect(),
        })?;

        Ok(())
    }

    fn get_default(
        &self,
        name: &str,
        meta: &mut TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let manager = self.create_manager(&self.get_path(name), meta)?;
        let default_value = manager.find(&Pos(0, 0))?.unwrap();
        std::mem::drop(manager);
        meta.field
            .iter_mut()
            .zip(default_value.data)
            .for_each(|(item, value)| {
                item.default_value = value;
            });
        Ok(())
    }

    pub fn list_table(&self) -> Result<Vec<Vec<DataValue>>, Box<dyn std::error::Error>> {
        let write_meta = gen_col_schema();
        let manager = self.create_manager(TABLE_COL_NAME, &write_meta)?;
        let mut res = manager
            .iter()
            .filter(|x| x.data[0] == DataValue::VarStr(self.database.clone()))
            .map(|x| match x.data[1].clone() {
                DataValue::VarStr(x) => x,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        res.sort();
        res.dedup();
        Ok(res
            .into_iter()
            .map(|x| vec![DataValue::VarStr(x)])
            .collect())
    }

    pub fn create_table(
        &self,
        table_name: &str,
        meta: &TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.create_col(table_name, meta)?;
        self.create_fk(table_name, meta)?;
        self.create_idx(table_name, meta)?;
        self.set_default(table_name, meta)?;
        Ok(())
    }

    pub fn delete_table(&self, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        {
            let write_meta = gen_col_schema();
            let manager = self.create_manager(TABLE_COL_NAME, &write_meta)?;
            manager.delete_where(|x| {
                x.data[0] == DataValue::VarStr(self.database.clone())
                    && x.data[1] == DataValue::VarStr(table_name.to_string())
            })?;
        }
        {
            let write_meta = gen_fk_schema();
            let manager = self.create_manager(TABLE_FK_NAME, &write_meta)?;
            manager.delete_where(|x| {
                x.data[0] == DataValue::VarStr(self.database.clone())
                    && x.data[2] == DataValue::VarStr(table_name.to_string())
            })?;
        }
        {
            let write_meta = gen_idx_schema();
            let manager = self.create_manager(TABLE_IDX_NAME, &write_meta)?;
            manager.delete_where(|x| {
                x.data[0] == DataValue::VarStr(self.database.clone())
                    && x.data[1] == DataValue::VarStr(table_name.to_string())
            })?;
        }
        Ok(())
    }

    pub fn update_table(
        &self,
        table_name: &str,
        meta: &TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.delete_table(table_name)?;
        self.create_table(table_name, meta)
    }

    pub fn get_db(&self) -> String {
        self.database.clone()
    }
    pub fn get_path(&self, name: &str) -> String {
        format!("data.{}.{}.table", self.database, name)
    }
    pub fn get_index_path(&self, name: &str, idx: &IndexOptions) -> String {
        match idx {
            IndexOptions::PrimaryKey => format!("pki.{}.{}.table", self.database, name),
            IndexOptions::Index(x) => format!("index.{}.{}.{}.table", self.database, name, x),
        }
    }
}
const TABLE_COL_NAME: &str = "global.col.table";
const TABLE_FK_NAME: &str = "global.fk.table";
const TABLE_IDX_NAME: &str = "global.idx.table";

const P_NULLABLE: i32 = 0x10000;
const P_PRIMARY: i32 = 0x20000;

fn get_property(item: &DataColumn) -> i32 {
    (if item.is_nullable { P_NULLABLE } else { 0 })
        | (if item.is_primary { P_PRIMARY } else { 0 })
        | (match item.data_type {
            DataType::Int => 0,
            DataType::Float => 1,
            DataType::VarStr(x) => 2 + (x as i32),
        })
}

fn gen_col_schema() -> TableMeta {
    TableMeta {
        id: 0,
        field: vec![
            DataColumn {
                name: String::from("db_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("tb_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("col_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("property"),
                data_type: DataType::Int,
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("order"),
                data_type: DataType::Int,
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
        ],
        foreign_key: vec![],
        index: vec![],
    }
}

fn gen_fk_schema() -> TableMeta {
    TableMeta {
        id: 0,
        field: vec![
            DataColumn {
                name: String::from("db_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("fk_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("tb_from_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("tb_to_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("col_from_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("col_to_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("id"),
                data_type: DataType::Int,
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
        ],
        foreign_key: vec![],
        index: vec![],
    }
}

fn gen_idx_schema() -> TableMeta {
    TableMeta {
        id: 0,
        field: vec![
            DataColumn {
                name: String::from("db_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("tb_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("idx"),
                data_type: DataType::Int,
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("col_name"),
                data_type: DataType::VarStr(32),
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
            DataColumn {
                name: String::from("order"),
                data_type: DataType::Int,
                is_primary: false,
                is_nullable: false,
                default_value: DataValue::Null,
            },
        ],
        foreign_key: vec![],
        index: vec![],
    }
}
