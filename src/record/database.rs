use crate::filesystem::LRUBufPageManager;
use crate::util::error::Error as DBError;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use super::column::{DataColumn, TableMeta};
use super::data::{DataEntry, DataValue};
use super::table::{Pos, RecordManager};

fn gen_db_table_schema() -> TableMeta {
    TableMeta {
        id: 0,
        field: vec![DataColumn {
            name: String::from("database"),
            data_type: super::column::DataType::VarStr(32),
            is_primary: false,
            is_nullable: false,
            default_value: super::data::DataValue::Null,
        }],
        foreign_key: vec![],
        index: vec![],
    }
}
const DATABASE_FILENAME: &str = "global.table";

pub struct DatabaseTable {
    /// Fd of the metadata file
    path: String,
    buf: Rc<RefCell<LRUBufPageManager>>,
    schema: TableMeta,
}

impl DatabaseTable {
    /// create or open a database table
    pub fn new(
        path: &str,
        buf_i: &Rc<RefCell<LRUBufPageManager>>,
    ) -> Result<DatabaseTable, Box<dyn std::error::Error>> {
        let mut path = PathBuf::from(path);
        path.push(DATABASE_FILENAME);
        let path_str = match path.to_str() {
            Some(x) => x,
            None => return Err(Box::new(DBError::new("path error"))),
        };
        let schema = gen_db_table_schema();
        let buf = Rc::clone(buf_i);

        // First start a DB connection just to make sure everything is OK.
        RecordManager::new(path_str, buf_i, &schema)?;

        Ok(DatabaseTable {
            schema,
            buf,
            path: String::from(path_str),
        })
    }

    /// create a database
    /// returns id(pos) of the database
    pub fn create_database(&self, name: &str) -> Result<Pos, Box<dyn std::error::Error>> {
        let mut manager = RecordManager::new(&self.path, &self.buf, &self.schema)?;
        if manager
            .iter()
            .any(|x| x.data[0] == DataValue::VarStr(name.to_string()))
        {
            return Err(Box::new(DBError::new("database already exists!")));
        }
        manager.insert(&DataEntry {
            id: Pos(0, 0),
            data: vec![DataValue::VarStr(String::from(name))],
        })
    }

    pub fn show_databases(&self) -> Result<Vec<DataValue>, Box<dyn std::error::Error>> {
        let manager = RecordManager::new(&self.path, &self.buf, &self.schema)?;

        let iter = manager.iter();
        let res = iter.map(|mut x| std::mem::take(&mut x.data[0])).collect();

        Ok(res)
    }

    pub fn drop_database(&self, name: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let manager = RecordManager::new(&self.path, &self.buf, &self.schema)?;

        manager.delete_where(|x| x.data[0] == DataValue::VarStr(String::from(name)))
    }

    pub fn find_database(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let manager = RecordManager::new(&self.path, &self.buf, &self.schema)?;
        Ok(manager
            .iter()
            .any(|x| x.data[0] == DataValue::VarStr(name.to_string())))
    }
}
