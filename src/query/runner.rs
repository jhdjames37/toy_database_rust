use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::rc::Rc;

use super::builder::{Field, Query};
use super::record_selector::check_use_index;
use super::select_validator::SelectValidator;
use super::types::{Cmp, FilterRow, FilterRowType, SelectTree, SelectTreeVisitor, Selector};
use crate::filesystem::LRUBufPageManager as BufPageManager;
use crate::index::manager::{IndexIterator, IndexManager, IndexOptions};
use crate::query::record_selector::RecordQuery;
use crate::record::column::{DataColumn, DataType, ForeignKey, Index, TableMeta, TableMetaManager};
use crate::record::data::{DataEntry, DataValue};
use crate::record::database::DatabaseTable;
use crate::record::table::{Pos, RecordManager};
use crate::util::error::Error as DBError;
use crate::util::regex::Matcher;

pub struct QueryRunner {
    db_table: Rc<RefCell<DatabaseTable>>,
    buf_page: Rc<RefCell<BufPageManager>>,
    base_path: String,
    db_ctx: Rc<RefCell<Option<TableMetaManager>>>,
}

pub enum QueryResult {
    Nop(i32),
    PartialSuccess(i32, Option<Box<dyn std::error::Error>>),
    Result(Vec<String>, Vec<Vec<DataValue>>),
    MetaResult(TableMeta),
}

type FiltFn = dyn Fn(&Vec<DataValue>) -> bool;

impl QueryRunner {
    pub fn run_cmd(&mut self, query: Query) -> Result<QueryResult, Box<dyn std::error::Error>> {
        match query {
            Query::CreateDb(name) => {
                let db_table = self.db_table.try_borrow_mut()?;
                db_table.create_database(&name)?;
                Ok(QueryResult::Nop(1))
            }
            Query::ListDb() => {
                let db_table = self.db_table.try_borrow_mut()?;
                Ok(QueryResult::Result(
                    vec![String::from("name")],
                    db_table
                        .show_databases()?
                        .into_iter()
                        .map(|x| vec![x])
                        .collect(),
                ))
            }
            Query::DropDb(name) => {
                let mut ctx = Rc::new(RefCell::new(Some(TableMetaManager::new(
                    &self.base_path,
                    &self.buf_page,
                    &name,
                ))));

                // switch context temporily for delete tables
                std::mem::swap(&mut ctx, &mut self.db_ctx);
                let tables = (*self.db_ctx.try_borrow_mut()?)
                    .as_ref()
                    .unwrap()
                    .list_table()?;

                for item in tables {
                    if let DataValue::VarStr(x) = &item[0] {
                        self.drop_table(x.to_string(), false)?;
                    }
                }

                std::mem::swap(&mut ctx, &mut self.db_ctx);
                if self.get_db() == Some(name.clone()) {
                    self.db_ctx = Rc::new(RefCell::new(None))
                }

                let db_table = self.db_table.try_borrow_mut()?;
                match db_table.drop_database(&name)? {
                    0 => Err(Box::new(DBError::new("No database found"))),
                    x => Ok(QueryResult::Nop(x)),
                }
            }
            Query::UseDb(name) => {
                let db_table = self.db_table.try_borrow()?;
                if !db_table.find_database(&name)? {
                    return Err(Box::new(DBError::new("database not found")));
                }
                self.db_ctx = Rc::new(RefCell::new(Some(TableMetaManager::new(
                    &self.base_path,
                    &self.buf_page,
                    &name,
                ))));
                Ok(QueryResult::Nop(0))
            }
            Query::CreateTable((name, fields)) => {
                self.create_table(name, fields)?;
                Ok(QueryResult::Nop(1))
            }
            Query::DescTable(name) => Ok(QueryResult::MetaResult(self.get_meta(&name)?)),
            Query::ListTable() => {
                let ctx = self.db_ctx.try_borrow()?;
                if let Some(ctx) = &*ctx {
                    Ok(QueryResult::Result(
                        vec!["Table".to_string()],
                        ctx.list_table()?,
                    ))
                } else {
                    Err(Box::new(DBError::new("no database selected!")))
                }
            }
            Query::DropTable(name) => {
                self.drop_table(name, true)?;
                Ok(QueryResult::Nop(1))
            }

            Query::CreateForeignKey(name, from, from_col, to, to_col) => {
                self.create_fk(name, from, from_col, to, to_col)?;
                Ok(QueryResult::Nop(1))
            }
            Query::DropForeignKey(from, name) => {
                self.remove_fk(from, name)?;
                Ok(QueryResult::Nop(1))
            }
            Query::CreatePrimaryKey(name, col) => {
                self.create_pk(name, col)?;
                Ok(QueryResult::Nop(1))
            }
            Query::DropPrimaryKey(name) => {
                self.remove_pk(name)?;
                Ok(QueryResult::Nop(1))
            }
            Query::CreateIndex(name, col) => {
                self.create_idx(name, col)?;
                Ok(QueryResult::Nop(1))
            }
            Query::DropIndex(name, col) => {
                self.remove_index(name, col)?;
                Ok(QueryResult::Nop(1))
            }

            Query::InsertTable(name, value) => {
                let (res, e) = self.insert(name, value)?;
                Ok(QueryResult::PartialSuccess(res, e))
            }
            Query::SelectTable(tree) => self.select(tree),
            Query::DeleteTable(name, filt) => Ok(QueryResult::Nop(self.delete(name, filt)?)),
            Query::UpdateTable(name, filt, upd) => {
                Ok(QueryResult::Nop(self.update(name, filt, upd)?))
            }
            Query::LoadData(name, file) => Ok(QueryResult::Nop(self.load_data(name, file)?)),

            // Errors
            Query::Err(x) => Err(Box::new(DBError::new(&x))),
            _ => Err(Box::new(DBError::new("Unknown instruction"))),
        }
    }

    pub fn get_db(&self) -> Option<String> {
        (*self.db_ctx.borrow()).as_ref().map(|x| x.get_db())
    }

    fn load_data(&self, name: String, filename: String) -> Result<i32, Box<dyn std::error::Error>> {
        let meta = self.get_meta(&name)?;
        let mut res = 0;
        let path = self.get_path(&name)?;

        let mut manager = RecordManager::new(&path, &self.buf_page, &meta)?;
        let mut index = self.get_all_index(&name, &meta)?;

        let data_file = File::open(
            filename
                .strip_prefix("'")
                .unwrap()
                .strip_suffix("'")
                .unwrap(),
        )?;
        let mut reader = BufReader::new(data_file);
        let mut buffer = String::new();

        while reader.read_line(&mut buffer)? > 0 {
            let item = buffer
                .split(',')
                .zip(&meta.field)
                .map(|(data, field)| {
                    let data = data.trim();
                    match field.data_type {
                        DataType::Int => DataValue::Int(data.parse().unwrap()),
                        DataType::Float => DataValue::Float(data.parse().unwrap()),
                        DataType::VarStr(_) => DataValue::VarStr(data.to_string()),
                    }
                })
                .collect();

            let data = DataEntry {
                id: Pos(0, 0),
                data: item,
            };
            res += 1;
            let pos = manager.insert(&data)?;

            for index_manager in &mut index {
                index_manager.insert(&data.data, &pos)?;
            }
            buffer.clear();
        }

        Ok(res)
    }

    /// check whether the data consists with metadata
    fn check_consistency(&self, meta: &TableMeta, data: &DataEntry) -> Result<(), DBError> {
        let data = &data.data;

        if meta.field.len() != data.len() {
            return Err(DBError::new("value length mismatch!"));
        }

        for i in meta.field.iter().map(|x| &x.data_type).zip(data.iter()) {
            match i {
                (DataType::Int, DataValue::Int(_))
                | (DataType::Float, DataValue::Float(_))
                | (_, DataValue::Null) => {
                    // do nothing
                }
                (DataType::VarStr(len), DataValue::VarStr(s)) => {
                    if s.len() > *len as usize {
                        return Err(DBError::new("String length is larger than field"));
                    }
                }
                _ => {
                    return Err(DBError::new("Mismatched Types!"));
                }
            }
        }
        Ok(())
    }

    fn get_path(&self, name: &str) -> Result<String, DBError> {
        if let Some(db) = &*self.db_ctx.borrow_mut() {
            let mut path = PathBuf::from(&self.base_path);
            path.push(db.get_path(name));
            match path.to_str() {
                Some(x) => Ok(x.to_string()),
                None => Err(DBError::new("path error")),
            }
        } else {
            Err(DBError::new("database not found"))
        }
    }

    fn get_index_path(&self, name: &str, idx: &IndexOptions) -> Result<String, DBError> {
        if let Some(db) = &*self.db_ctx.borrow_mut() {
            let mut path = PathBuf::from(&self.base_path);
            path.push(db.get_index_path(name, idx));
            match path.to_str() {
                Some(x) => Ok(x.to_string()),
                None => Err(DBError::new("path error")),
            }
        } else {
            Err(DBError::new("database not found"))
        }
    }

    fn create_table(
        &mut self,
        name: String,
        fields: Vec<Field>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // check all indentifiers are correct
        // 1. no duplicated columns
        let mut idents: HashSet<String> = HashSet::new();
        let mut meta = TableMeta {
            id: 0,
            field: vec![],
            foreign_key: vec![],
            index: vec![],
        };

        for field in &fields {
            match field {
                Field::Column(col_name, data_type, is_not_null, default_value) => {
                    if idents.contains(col_name) {
                        return Err(Box::new(DBError::new(&format!(
                            "Field {} duplicates!",
                            col_name
                        ))));
                    }
                    idents.insert(col_name.clone());
                    match (&data_type, &default_value) {
                        (DataType::Int, DataValue::Int(_))
                        | (DataType::Float, DataValue::Float(_))
                        | (_, DataValue::Null) => {}
                        (DataType::VarStr(x), DataValue::VarStr(y)) => {
                            if y.len() > *x as usize {
                                return Err(Box::new(DBError::new(&format!(
                                    "Too long default value in column {}",
                                    col_name
                                ))));
                            }
                        }
                        _ => {
                            return Err(Box::new(DBError::new(&format!(
                                "Invalid default value in column {}",
                                col_name
                            ))));
                        }
                    }
                    meta.field.push(DataColumn {
                        name: col_name.clone(),
                        data_type: data_type.clone(),
                        is_primary: false,
                        is_nullable: !is_not_null,
                        default_value: default_value.clone(),
                    });
                }
                _ => {}
            }
        }
        // 2. process PK and FK
        for field in &fields {
            match field {
                Field::PrimaryKey(cols) => {
                    for col in cols {
                        if !idents.contains(col) {
                            return Err(Box::new(DBError::new(&format!(
                                "Primary key {} not found!",
                                col
                            ))));
                        }
                        meta.field.iter_mut().for_each(|item| {
                            if item.name == *col {
                                item.is_primary = true
                            }
                        })
                    }
                }
                Field::ForeignKey(fk_name, cols, ref_name, ref_cols) => {
                    if cols.len() != ref_cols.len() {
                        return Err(Box::new(DBError::new(
                            "Foreign Key column number not match!",
                        )));
                    }
                    let ref_meta = self.get_meta(ref_name)?;
                    // TODO: check out a column can appear in multiple FKs
                    for col in cols {
                        if !idents.contains(col) {
                            return Err(Box::new(DBError::new(&format!(
                                "Foreign key {} not found!",
                                col
                            ))));
                        }
                    }
                    for col in ref_cols {
                        if !ref_meta
                            .field
                            .iter()
                            .any(|x| x.name == *col && x.is_primary)
                        {
                            return Err(Box::new(DBError::new(&format!(
                                "Foreign key {}.{} not found, or is not a primary key!",
                                ref_name, col
                            ))));
                        }
                    }

                    let mut ref_cols_oredered = ref_cols.clone();
                    ref_cols_oredered.sort();

                    let mut ref_pk: Vec<_> = ref_meta
                        .field
                        .iter()
                        .filter(|x| x.is_primary)
                        .map(|x| x.name.clone())
                        .collect();
                    ref_pk.sort();
                    if ref_cols_oredered != ref_pk {
                        return Err(Box::new(DBError::new("Not all primary keys are used!")));
                    }

                    let id = meta.foreign_key.len() as u32;

                    let name = if !fk_name.is_empty() {
                        fk_name.clone()
                    } else {
                        let mut res = "".to_string();
                        for idx in 0.. {
                            res = format!("fk_{}", idx);
                            if !meta.foreign_key.iter().any(|x| x.name == res) {
                                break;
                            }
                        }
                        res
                    };

                    if meta.foreign_key.iter().any(|x| x.name == name) {
                        return Err(Box::new(DBError::new("Foreign key name duplicates!")));
                    }
                    meta.foreign_key.push(ForeignKey {
                        name,
                        from: cols.clone(),
                        table: ref_name.clone(),
                        to: ref_cols.clone(),
                        id,
                    });
                }
                _ => {}
            }
        }
        // 3. write into database
        if let Some(ctx) = &*self.db_ctx.try_borrow_mut()? {
            ctx.create_table(&name, &meta)?;
        } else {
            return Err(Box::new(DBError::new("no database selected!")));
        }

        // 4. init PK index
        if meta.field.iter().any(|x| x.is_primary) {
            IndexManager::new(
                &self.get_index_path(&name, &IndexOptions::PrimaryKey)?,
                &self.buf_page,
                &meta,
                &IndexOptions::PrimaryKey,
            )?;
        }

        Ok(())
    }

    fn drop_table(&mut self, name: String, check: bool) -> Result<(), Box<dyn std::error::Error>> {
        let meta = self.get_meta(&name)?;
        if check {
            if !self.check_ref_fk(&name)?.is_empty() {
                return Err(Box::new(DBError::new("Foreign Key constraint exists!")));
            }
        }

        if meta.field.iter().any(|x| x.is_primary) {
            std::fs::remove_file(self.get_index_path(&name, &IndexOptions::PrimaryKey)?)?;
        }
        for item in meta.index {
            std::fs::remove_file(self.get_index_path(&name, &IndexOptions::Index(item.id))?)?;
        }
        std::fs::remove_file(self.get_path(&name)?)?;

        if let Some(ctx) = &*self.db_ctx.try_borrow_mut()? {
            ctx.delete_table(&name)?;
        } else {
            unreachable!();
        }
        Ok(())
    }

    fn create_fk(
        &self,
        name: String,
        from: String,
        from_col: Vec<String>,
        to: String,
        to_col: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if from_col.len() != to_col.len() {
            return Err(Box::new(DBError::new(&format!(
                "Reference length mismatch!"
            ))));
        }
        let mut from_meta = self.get_meta(&from)?;
        let to_meta = self.get_meta(&to)?;

        if let Some(x) = from_col
            .iter()
            .find(|x| !from_meta.field.iter().any(|y| y.name == **x))
        {
            return Err(Box::new(DBError::new(&format!(
                "Column {from}.{x} not found"
            ))));
        }
        if let Some(x) = to_col
            .iter()
            .find(|x| !to_meta.field.iter().any(|y| y.name == **x))
        {
            return Err(Box::new(DBError::new(&format!(
                "Column {to}.{x} not found"
            ))));
        }
        if let Some(x) = to_col
            .iter()
            .find(|x| !to_meta.field.iter().any(|y| y.name == **x && y.is_primary))
        {
            return Err(Box::new(DBError::new(&format!(
                "Column {to}.{x} is not primary key"
            ))));
        }

        {
            let mut ref_cols_oredered = to_col.clone();
            ref_cols_oredered.sort();

            let mut ref_pk: Vec<_> = to_meta
                .field
                .iter()
                .filter(|x| x.is_primary)
                .map(|x| x.name.clone())
                .collect();
            ref_pk.sort();
            if ref_cols_oredered != ref_pk {
                return Err(Box::new(DBError::new("Not all primary keys are used!")));
            }
        }

        let from_idx: Vec<_> = from_col
            .iter()
            .map(|x| {
                from_meta
                    .field
                    .iter()
                    .enumerate()
                    .find(|(_, y)| y.name == **x)
                    .unwrap()
                    .0
            })
            .collect();
        let to_map: Vec<_> = to_meta
            .field
            .iter()
            .filter(|x| x.is_primary)
            .map(|x| {
                to_col
                    .iter()
                    .enumerate()
                    .find(|(_, y)| **y == x.name)
                    .unwrap()
                    .0
            })
            .collect();
        /*
            .iter()
            .map(|x| {
                to_meta
                    .field
                    .iter()
                    .enumerate()
                    .find(|(_, y)| y.name == **x)
                    .unwrap()
                    .0
            })
            .collect();
        */
        let from_path = self.get_path(&from)?;
        let from_manager = RecordManager::new(&from_path, &self.buf_page, &from_meta)?;
        let to_idx_path = self.get_index_path(&to, &IndexOptions::PrimaryKey)?;
        let to_idx_manager = IndexManager::new(
            &to_idx_path,
            &self.buf_page,
            &to_meta,
            &IndexOptions::PrimaryKey,
        )?;

        for from_item in from_manager.iter() {
            let from_data: Vec<_> = from_idx
                .iter()
                .map(|x| from_item.data[*x].clone())
                .collect();
            let from_data_mapped: Vec<_> = to_map.iter().map(|x| from_data[*x].clone()).collect();
            if !to_idx_manager.find_key(&from_data_mapped)?.is_some() {
                return Err(Box::new(DBError::new("Cannot find Foreign key reference")));
            }
        }

        std::mem::drop(from_manager);

        let mut id = 0;
        while from_meta.foreign_key.iter().any(|x| x.id == id) {
            id += 1;
        }

        let name = if !name.is_empty() {
            name
        } else {
            let mut res = "".to_string();
            for idx in 0.. {
                res = format!("fk_{}", idx);
                if !from_meta.foreign_key.iter().any(|x| x.name == res) {
                    break;
                }
            }
            res
        };

        if from_meta.foreign_key.iter().any(|x| x.name == name) {
            return Err(Box::new(DBError::new("Foreign key name duplicates!")));
        }

        from_meta.foreign_key.push(ForeignKey {
            name,
            from: from_col,
            table: to,
            to: to_col,
            id,
        });
        self.update_table(&from, &from_meta)?;
        Ok(())
    }

    fn remove_fk(&self, from: String, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut meta = self.get_meta(&from)?;
        if !meta.foreign_key.iter().any(|x| x.name == name) {
            return Err(Box::new(DBError::new(&format!(
                "Foreign key {name} not found"
            ))));
        }
        meta.foreign_key = meta
            .foreign_key
            .into_iter()
            .filter(|x| x.name != name)
            .collect();
        self.update_table(&from, &meta)?;
        Ok(())
    }

    fn create_pk(&self, table: String, col: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut meta = self.get_meta(&table)?;
        if meta.field.iter().any(|x| x.is_primary) {
            return Err(Box::new(DBError::new("Primary key already exists!")));
        }
        if let Some(x) = col
            .iter()
            .find(|x| meta.field.iter().find(|y| y.name == **x).is_none())
        {
            return Err(Box::new(DBError::new(&format!("Column {x} not found!"))));
        }

        // check data constraint
        let idx: Vec<_> = meta
            .field
            .iter()
            .enumerate()
            .filter(|(_, x)| col.contains(&x.name))
            .map(|(x, _)| x)
            .collect();
        // For convenieness, Load all item in the Memory
        /*
        let mut set: Vec<Vec<DataValue>> = Vec::new();
        for item in manager.iter() {
            let data = idx.iter().map(|x| item.data[*x].clone()).collect();
            if set.contains(&data) {
                return Err(Box::new(DBError::new(
                    "Data not satisfy primary key constraints!",
                )));
            }
            set.push(data);
        }
         */
        meta.field.iter_mut().for_each(|x| {
            if col.contains(&x.name) {
                x.is_primary = true;
            }
        });

        let mut idx_manager = IndexManager::new(
            &self.get_index_path(&table, &IndexOptions::PrimaryKey)?,
            &self.buf_page,
            &meta,
            &IndexOptions::PrimaryKey,
        )?;

        let r_path = self.get_path(&table)?;
        let manager = RecordManager::new(&r_path, &self.buf_page, &meta)?;
        if let Err(e) = (|| -> Result<(), Box<dyn std::error::Error>> {
            for item in manager.iter() {
                if idx_manager.find(&item.data)?.is_some() {
                    return Err(Box::new(DBError::new(
                        "Data not satisfy primary key constraints!",
                    )));
                }
                idx_manager.insert(&item.data, &item.id)?;
            }
            Ok(())
        })() {
            // Force close file
            std::mem::drop(idx_manager);
            std::fs::remove_file(&self.get_index_path(&table, &IndexOptions::PrimaryKey)?)?;
            return Err(e);
        }
        self.update_table(&table, &meta)?;
        Ok(())
    }

    fn update_table(&self, name: &str, meta: &TableMeta) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ctx) = &*self.db_ctx.try_borrow_mut()? {
            ctx.update_table(&name, &meta)
        } else {
            return Err(Box::new(DBError::new("no database selected!")));
        }
    }

    fn remove_pk(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut meta = self.get_meta(&name)?;
        if !self.check_ref_fk(&name)?.is_empty() {
            return Err(Box::new(DBError::new("Foreign Key constraint exists!")));
        }

        meta.field.iter_mut().for_each(|item| {
            item.is_primary = false;
        });

        self.update_table(&name, &meta)?;

        std::fs::remove_file(&self.get_index_path(&name, &IndexOptions::PrimaryKey)?)?;

        Ok(())
    }

    fn check_ref_fk(
        &self,
        name: &str,
    ) -> Result<Vec<(String, ForeignKey)>, Box<dyn std::error::Error>> {
        if let Some(ctx) = &*self.db_ctx.try_borrow_mut()? {
            ctx.find_fk_constraint(name)
        } else {
            return Err(Box::new(DBError::new("no database selected!")));
        }
    }

    fn create_idx(&self, name: String, col: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut meta = self.get_meta(&name)?;
        let mut ids: HashSet<u32> = HashSet::new();
        for item in &meta.index {
            ids.insert(item.id);
            if item.col == col {
                return Err(Box::new(DBError::new("Index already exists!")));
            }
        }
        for item in &col {
            if !meta.field.iter().find(|x| x.name == *item).is_some() {
                return Err(Box::new(DBError::new(&format!(
                    "Column {} not found!",
                    item
                ))));
            }
        }
        let mut id = 0;
        while ids.contains(&id) {
            id += 1;
        }
        meta.index.push(Index { col, id });

        // update metadata
        if let Some(ctx) = &*self.db_ctx.try_borrow_mut()? {
            ctx.update_table(&name, &meta)?;
        } else {
            return Err(Box::new(DBError::new("no database selected!")));
        }

        // create index table
        let mut idx_manager = IndexManager::new(
            &self.get_index_path(&name, &IndexOptions::Index(id))?,
            &self.buf_page,
            &meta,
            &IndexOptions::Index(id),
        )?;

        let r_path = self.get_path(&name)?;
        let manager = RecordManager::new(&r_path, &self.buf_page, &meta)?;
        for item in manager.iter() {
            idx_manager.insert(&item.data, &item.id)?;
        }

        Ok(())
    }

    fn remove_index(
        &self,
        name: String,
        col: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut meta = self.get_meta(&name)?;
        let (idx, id) =
            if let Some((t, x)) = meta.index.iter().enumerate().find(|(_, x)| x.col == col) {
                (t, x.id)
            } else {
                return Err(Box::new(DBError::new("index not found")));
            };
        meta.index.remove(idx);

        if let Some(ctx) = &*self.db_ctx.try_borrow_mut()? {
            ctx.update_table(&name, &meta)?;
        } else {
            return Err(Box::new(DBError::new("no database selected!")));
        }

        std::fs::remove_file(self.get_index_path(&name, &IndexOptions::Index(id))?)?;
        Ok(())
    }

    fn get_all_index(
        &self,
        name: &String,
        meta: &TableMeta,
    ) -> Result<Vec<IndexManager>, Box<dyn std::error::Error>> {
        let mut index = vec![];
        if meta.field.iter().any(|x| x.is_primary) {
            index.push(IndexManager::new(
                &self.get_index_path(&name, &IndexOptions::PrimaryKey)?,
                &self.buf_page,
                &meta,
                &IndexOptions::PrimaryKey,
            )?);
        }
        for item in &meta.index {
            index.push(IndexManager::new(
                &self.get_index_path(&name, &IndexOptions::Index(item.id))?,
                &self.buf_page,
                &meta,
                &IndexOptions::Index(item.id),
            )?);
        }
        Ok(index)
    }
    fn insert(
        &self,
        name: String,
        data: Vec<Vec<DataValue>>,
    ) -> Result<(i32, Option<Box<dyn std::error::Error>>), Box<dyn std::error::Error>> {
        let meta = self.get_meta(&name)?;
        let mut res = 0;
        let path = self.get_path(&name)?;

        let mut manager = RecordManager::new(&path, &self.buf_page, &meta)?;
        let mut index = self.get_all_index(&name, &meta)?;

        for item in data {
            let data = DataEntry {
                id: Pos(0, 0),
                data: item,
            };
            if let Err(e) = (|| -> Result<(), Box<dyn std::error::Error>> {
                self.check_consistency(&meta, &data)?;
                self.check_primary_key(&manager, &data.data, &meta)?;
                self.check_foreign_key(&data.data, &meta)?;
                self.check_not_null(&meta, &data.data)?;
                Ok(())
            })() {
                return Ok((res, Some(e)));
            }
            res += 1;
            let pos = manager.insert(&data)?;

            for index_manager in &mut index {
                index_manager.insert(&data.data, &pos)?;
            }
        }

        Ok((res, None))
    }
    fn delete(
        &self,
        name: String,
        filter: Vec<FilterRow>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let meta = self.get_meta(&name)?;
        let path = self.get_path(&name)?;

        let mut del_pos: Vec<DataEntry> = vec![];
        // check filter
        let filt_fn = self.check_filter_simple(&name, &meta, &filter)?;
        let refs = self.check_ref_fk(&name)?;

        let del_fn = |data: DataEntry| {
            if filt_fn.iter().all(|f| f(&data.data)) {
                // Check constraints

                del_pos.push(data);
            }
        };

        if let Some((idx, lower, upper)) = self.get_filter_bounds(&filter, &meta) {
            println!(
                "Use {}!",
                if let IndexOptions::PrimaryKey = idx {
                    "Primary Key Index"
                } else {
                    "Index"
                }
            );
            let manager = RecordManager::new(&path, &self.buf_page, &meta)?;
            IndexIterator::new(
                &self.get_index_path(&name, &idx)?,
                &self.buf_page,
                &meta,
                &idx,
                &lower,
                &upper,
                manager,
            )?
            .for_each(del_fn);
        } else {
            let manager = RecordManager::new(&path, &self.buf_page, &meta)?;
            manager.iter().for_each(del_fn);
        };

        let manager = RecordManager::new(&path, &self.buf_page, &meta)?;

        self.check_del_fk_constraint(&refs, &meta, &del_pos, &vec![])?;

        let mut index = self.get_all_index(&name, &meta)?;
        for data in &del_pos {
            manager.delete(&data.id)?;
            for ind in &mut index {
                ind.remove(&data.data, &data.id)?;
            }
        }

        Ok(del_pos.len() as i32)
    }
    fn check_del_fk_constraint(
        &self,
        refs: &Vec<(String, ForeignKey)>,
        meta: &TableMeta,
        data: &Vec<DataEntry>,
        upd_data: &Vec<DataEntry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for ref_item in refs {
            let from_name = &ref_item.0;
            let path = self.get_path(from_name)?;
            let from_meta = self.get_meta(from_name)?;
            let manager = RecordManager::new(&path, &self.buf_page, &from_meta)?;
            let from_idx: Vec<_> = ref_item
                .1
                .from
                .iter()
                .map(|x| {
                    for (i, data) in from_meta.field.iter().enumerate() {
                        if data.name == *x {
                            return i;
                        }
                    }
                    unreachable!();
                })
                .collect();
            let to_idx: Vec<_> = ref_item
                .1
                .to
                .iter()
                .map(|x| {
                    for (i, data) in meta.field.iter().enumerate() {
                        if data.name == *x {
                            return i;
                        }
                    }
                    unreachable!();
                })
                .collect();
            for from_data in manager.iter() {
                let match_data = |to_data: &DataEntry| {
                    from_idx
                        .iter()
                        .map(|id| &from_data.data[*id])
                        .eq(to_idx.iter().map(|id| &to_data.data[*id]))
                };
                // There is a referenced key deleting
                if data.iter().any(match_data) &&
                // and also there isn't an insert key in the updated data (if any)
                    !upd_data.iter().any(match_data)
                {
                    return Err(Box::new(DBError::new("Foreign key Violation")));
                }
            }
        }

        Ok(())
    }

    fn update(
        &self,
        name: String,
        filter: Vec<FilterRow>,
        update: Vec<(String, DataValue)>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let meta = self.get_meta(&name)?;
        let path = self.get_path(&name)?;

        let mut del_pos: Vec<DataEntry> = vec![];
        let mut upd_pos: Vec<DataEntry> = vec![];
        // check filter
        let filt_fn = self.check_filter_simple(&name, &meta, &filter)?;
        let refs = self.check_ref_fk(&name)?;

        // check update statements
        let mut upd_ops: Vec<(usize, DataValue)> = vec![];
        let mut pk_upd = false;
        for upd_item in update {
            let mut found = false;
            for (i, data) in meta.field.iter().enumerate() {
                if data.name == upd_item.0 {
                    match (&data.data_type, &upd_item.1) {
                        (_, DataValue::Null)
                        | (DataType::Int, DataValue::Int(_))
                        | (DataType::Float, DataValue::Float(_))
                        | (DataType::VarStr(_), DataValue::VarStr(_)) => {}
                        _ => {
                            return Err(Box::new(DBError::new("Type violation!")));
                        }
                    }
                    upd_ops.push((i, upd_item.1));
                    found = true;
                    if data.is_primary {
                        pk_upd = true;
                    }
                    break;
                }
            }
            if !found {
                return Err(Box::new(DBError::new(&format!(
                    "Field {} not found!",
                    upd_item.0
                ))));
            }
        }

        let del_fn = |data: DataEntry| {
            if filt_fn.iter().all(|f| f(&data.data)) {
                let mut upd_data = data.clone();
                for (idx, value) in &upd_ops {
                    upd_data.data[*idx] = value.clone();
                }
                del_pos.push(data);
                upd_pos.push(upd_data);
            }
        };

        if let Some((idx, lower, upper)) = self.get_filter_bounds(&filter, &meta) {
            println!(
                "Use {}!",
                if let IndexOptions::PrimaryKey = idx {
                    "Primary Key Index"
                } else {
                    "Index"
                }
            );
            let manager = RecordManager::new(&path, &self.buf_page, &meta)?;
            IndexIterator::new(
                &self.get_index_path(&name, &idx)?,
                &self.buf_page,
                &meta,
                &idx,
                &lower,
                &upper,
                manager,
            )?
            .for_each(del_fn);
        } else {
            let manager = RecordManager::new(&path, &self.buf_page, &meta)?;
            manager.iter().for_each(del_fn);
        };

        let manager = RecordManager::new(&path, &self.buf_page, &meta)?;
        // Check constriants
        // foreign key referenced
        if pk_upd {
            self.check_del_fk_constraint(&refs, &meta, &del_pos, &upd_pos)?;
        }
        // foreign key referencing
        // + not null
        for item in &upd_pos {
            self.check_foreign_key(&item.data, &meta)?;
            self.check_not_null(&meta, &item.data)?;
        }
        // primary key
        'pk_check: {
            if !pk_upd {
                break 'pk_check;
            }
            let pk_idx: Vec<_> = meta
                .field
                .iter()
                .enumerate()
                .filter(|(_, x)| x.is_primary)
                .map(|(x, _)| x)
                .collect();
            if pk_idx.is_empty() {
                break 'pk_check;
            }

            // first check out updated data itself has conflict
            let mut pk_item: Vec<Vec<DataValue>> = vec![];
            for item in &upd_pos {
                let pk: Vec<_> = pk_idx.iter().map(|x| item.data[*x].clone()).collect();
                if pk_item.contains(&pk) {
                    return Err(Box::new(DBError::new("Primary key violation!")));
                }
                pk_item.push(pk);
            }
            // check out original data conflicts with new data
            for item in manager.iter() {
                // filter out updated item
                if upd_pos.iter().any(|x| x.id == item.id) {
                    continue;
                }
                let pk = pk_idx.iter().map(|x| item.data[*x].clone()).collect();
                if pk_item.contains(&pk) {
                    return Err(Box::new(DBError::new("Pirmary key voilation!")));
                }
            }
        }

        let mut index = self.get_all_index(&name, &meta)?;
        for (data, upd_data) in del_pos.iter().zip(&upd_pos) {
            manager.update(&upd_data.id, upd_data)?;
            for ind in &mut index {
                ind.remove(&data.data, &data.id)?;
                ind.insert(&upd_data.data, &upd_data.id)?;
            }
        }
        Ok(del_pos.len() as i32)
    }

    fn get_filter_bounds(
        &self,
        filter: &Vec<FilterRow>,
        meta: &TableMeta,
    ) -> Option<(IndexOptions, Vec<Option<DataValue>>, Vec<Option<DataValue>>)> {
        let mut result: Vec<(bool, Selector, DataValue)> = vec![];
        for filt in filter {
            match &filt.kind {
                FilterRowType::CmpValue(col, op, value) => match op {
                    Cmp::Eq => {
                        result.push((false, col.clone(), value.clone()));
                        result.push((true, col.clone(), value.clone()));
                    }
                    Cmp::Lt | Cmp::Le => {
                        result.push((false, col.clone(), value.clone()));
                    }
                    Cmp::Ge | Cmp::Gt => {
                        result.push((true, col.clone(), value.clone()));
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let mut lower_bound: Vec<Option<DataValue>> = vec![None; meta.field.len()];
        let mut upper_bound: Vec<Option<DataValue>> = vec![None; meta.field.len()];
        for (is_lower, column, value) in &result {
            match column {
                Selector::Column(col) => {
                    for (i, item) in meta.field.iter().enumerate() {
                        if *col == item.name {
                            if *is_lower {
                                lower_bound[i] = Some(value.clone())
                            } else {
                                upper_bound[i] = Some(value.clone())
                            }
                            break;
                        }
                    }
                }
                Selector::TableColumn(_, col) => {
                    for (i, item) in meta.field.iter().enumerate() {
                        if *col == item.name {
                            if *is_lower {
                                lower_bound[i] = Some(value.clone())
                            } else {
                                upper_bound[i] = Some(value.clone())
                            }
                            break;
                        }
                    }
                }
            }
        }

        let mut options: Vec<_> = meta
            .index
            .iter()
            .map(|x| IndexOptions::Index(x.id))
            .collect();

        options.push(IndexOptions::PrimaryKey);
        match options
            .into_iter()
            .map(|x| {
                let result = check_use_index(&meta, &x, &lower_bound, &upper_bound);
                (x, result)
            })
            .max_by_key(|x| x.1)
        {
            Some((op, val)) if val > 0 => Some((op, lower_bound, upper_bound)),
            _ => None,
        }
    }

    fn check_filter_simple(
        &self,
        tab_name: &String,
        meta: &TableMeta,
        filter: &Vec<FilterRow>,
    ) -> Result<Vec<Box<FiltFn>>, Box<dyn std::error::Error>> {
        let check_col = |col: &Selector| -> Result<(usize, DataType), Box<dyn std::error::Error>> {
            let col_name = match col {
                Selector::Column(name) => name,
                Selector::TableColumn(tab, col) => {
                    if tab != tab_name {
                        return Err(Box::new(DBError::new(&format!("table {tab} not found"))));
                    }
                    col
                }
            };
            for (i, item) in meta.field.iter().enumerate() {
                if item.name == *col_name {
                    return Ok((i, item.data_type.clone()));
                }
            }
            Err(Box::new(DBError::new(&format!(
                "Column {col_name} not found!"
            ))))
        };

        let cmp = |x1: &DataValue, op: &Cmp, x2: &DataValue| match op {
            Cmp::Eq => x1 == x2,
            Cmp::Neq => x1 != x2,
            Cmp::Ge => x1 >= x2,
            Cmp::Gt => x1 > x2,
            Cmp::Le => x1 <= x2,
            Cmp::Lt => x1 < x2,
        };

        let mut res: Vec<Box<FiltFn>> = vec![];
        for f_item in filter {
            match &f_item.kind {
                FilterRowType::CmpColumn(col1, op, col2) => {
                    let (col1_idx, col1_type) = check_col(col1)?;
                    let (col2_idx, col2_type) = check_col(col2)?;
                    match (&col1_type, &col2_type) {
                        (DataType::Int, DataType::Int)
                        | (DataType::Float, DataType::Float)
                        | (DataType::VarStr(_), DataType::VarStr(_)) => {}
                        _ => {
                            return Err(Box::new(DBError::new("data type mismatch!")));
                        }
                    }
                    let fn_op = op.clone();
                    res.push(Box::new(move |data: &Vec<DataValue>| {
                        cmp(&data[col1_idx], &fn_op, &data[col2_idx])
                    }));
                }
                FilterRowType::CmpValue(col, op, val) => {
                    let (col_idx, col_type) = check_col(col)?;
                    match (col_type, &val) {
                        (DataType::Int, DataValue::Int(_))
                        | (DataType::Float, DataValue::Float(_))
                        | (DataType::VarStr(_), DataValue::VarStr(_)) => {}
                        _ => {
                            return Err(Box::new(DBError::new("data type mismatch!")));
                        }
                    }
                    let fn_op = op.clone();
                    let fn_val = val.clone();
                    res.push(Box::new(move |data: &Vec<DataValue>| {
                        cmp(&data[col_idx], &fn_op, &fn_val)
                    }));
                }
                FilterRowType::Like(col, val) => {
                    let (col_idx, col_type) = check_col(col)?;
                    match col_type {
                        DataType::VarStr(_) => {}
                        _ => {
                            return Err(Box::new(DBError::new("data type mismatch!")));
                        }
                    }
                    let matcher = Matcher::new(val.to_string());
                    res.push(Box::new(move |row: &Vec<DataValue>| match &row[col_idx] {
                        DataValue::Null => false,
                        DataValue::VarStr(x) => matcher.check(x),
                        _ => unreachable!(),
                    }))
                }
                FilterRowType::In(col, val) => {
                    let (col_idx, col_type) = check_col(col)?;
                    for item in val {
                        match (&col_type, &item) {
                            (DataType::Int, DataValue::Int(_))
                            | (DataType::Float, DataValue::Float(_))
                            | (DataType::VarStr(_), DataValue::VarStr(_)) => {}
                            _ => {
                                return Err(Box::new(DBError::new("data type mismatch!")));
                            }
                        }
                    }
                    let val = val.clone();
                    res.push(Box::new(move |row: &Vec<DataValue>| {
                        val.contains(&row[col_idx])
                    }))
                }
                FilterRowType::CmpSelect(col, op, sel) => {
                    let (col_idx, col_type) = check_col(col)?;
                    let n_op = op.clone();
                    let mut val = SelectValidator::new(&self.db_ctx);
                    let (_, sub_type) = val.visit(&sel)?;

                    if sub_type.len() != 1 {
                        return Err(Box::new(DBError::new("data type mismatch!")));
                    }

                    match (&col_type, &sub_type[0]) {
                        (DataType::Float, DataType::Float)
                        | (DataType::Int, DataType::Int)
                        | (DataType::VarStr(_), DataType::VarStr(_)) => {}
                        _ => {
                            return Err(Box::new(DBError::new("data type mismatch!")));
                        }
                    }

                    let mut run = RecordQuery::new(&self.base_path, &self.buf_page, &self.db_ctx)?;
                    let (_, mut data) = run.visit(&sel)?;
                    if let Some(value) = data.next() {
                        if data.next().is_some() {
                            return Err(Box::new(DBError::new("More than one data in sub select")));
                        }
                        res.push(Box::new(move |row: &Vec<DataValue>| {
                            cmp(&row[col_idx], &n_op, &value[0])
                        }))
                    } else {
                        return Err(Box::new(DBError::new("No Data in sub SELECT")));
                    }
                }
                FilterRowType::InSelect(col, sel) => {
                    let (col_idx, col_type) = check_col(col)?;
                    let mut val = SelectValidator::new(&self.db_ctx);
                    let (_, sub_type) = val.visit(&sel)?;

                    if sub_type.len() != 1 {
                        return Err(Box::new(DBError::new("data type mismatch!")));
                    }

                    match (&col_type, &sub_type[0]) {
                        (DataType::Float, DataType::Float)
                        | (DataType::Int, DataType::Int)
                        | (DataType::VarStr(_), DataType::VarStr(_)) => {}
                        _ => {
                            return Err(Box::new(DBError::new("data type mismatch!")));
                        }
                    }

                    let mut run = RecordQuery::new(&self.base_path, &self.buf_page, &self.db_ctx)?;
                    let (_, data) = run.visit(&sel)?;
                    let value: Vec<_> = data.map(|mut x| x.pop().unwrap()).collect();

                    res.push(Box::new(move |row: &Vec<DataValue>| {
                        value.contains(&row[col_idx])
                    }))
                }
            }
        }

        Ok(res)
    }

    fn check_not_null(
        &self,
        meta: &TableMeta,
        data: &Vec<DataValue>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (value, field) in data.iter().zip(&meta.field) {
            if *value == DataValue::Null && !field.is_nullable {
                return Err(Box::new(DBError::new("NULL constraint violation!")));
            }
        }
        Ok(())
    }

    fn check_primary_key(
        &self,
        manager: &RecordManager,
        data: &Vec<DataValue>,
        meta: &TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pk_idx: Vec<usize> = meta
            .field
            .iter()
            .enumerate()
            .filter(|(_, item)| item.is_primary)
            .map(|(id, _)| id)
            .collect();
        // No primary key
        if pk_idx.is_empty() {
            return Ok(());
        }
        let pk_data: Vec<DataValue> = pk_idx.iter().map(|id| data[*id].clone()).collect();
        if manager.iter().any(|table_item| {
            let table_item_pk: Vec<_> = pk_idx
                .iter()
                .map(|id| table_item.data[*id].clone())
                .collect();
            pk_data == table_item_pk
        }) {
            return Err(Box::new(DBError::new("Primary key violation!")));
        }

        Ok(())
    }

    fn check_foreign_key(
        &self,
        data: &Vec<DataValue>,
        meta: &TableMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for fk_item in meta.foreign_key.iter() {
            let fk_meta = self.get_meta(&fk_item.table)?;
            let fk_idx_path = self.get_index_path(&fk_item.table, &IndexOptions::PrimaryKey)?;
            let fk_idx_manager = IndexManager::new(
                &fk_idx_path,
                &self.buf_page,
                &fk_meta,
                &IndexOptions::PrimaryKey,
            )?;
            let fk_from_idx: Vec<usize> = fk_item
                .from
                .iter()
                .map(|x| {
                    let mut idx: usize = 0;
                    for (i, data) in meta.field.iter().enumerate() {
                        if data.name == *x {
                            idx = i;
                            break;
                        }
                    }
                    idx
                })
                .collect();
            let fk_map_idx: Vec<usize> = fk_meta
                .field
                .iter()
                .filter(|x| x.is_primary)
                .map(|x| {
                    let mut idx: usize = 0;
                    for (i, data) in fk_item.to.iter().enumerate() {
                        if *data == x.name {
                            idx = i;
                            break;
                        }
                    }
                    idx
                })
                .collect();
            let fk_from_data: Vec<DataValue> =
                fk_from_idx.iter().map(|id| data[*id].clone()).collect();
            let fk_mapped_data: Vec<_> = fk_map_idx
                .iter()
                .map(|id| fk_from_data[*id].clone())
                .collect();
            if !fk_idx_manager.find_key(&fk_mapped_data)?.is_some() {
                return Err(Box::new(DBError::new("Foreign Key violation!")));
            }
        }
        Ok(())
    }

    fn select(&self, tree: SelectTree) -> Result<QueryResult, Box<dyn std::error::Error>> {
        let mut val = SelectValidator::new(&self.db_ctx);
        val.run(&tree)?;

        let mut run = RecordQuery::new(&self.base_path, &self.buf_page, &self.db_ctx)?;
        run.run(&tree)
    }

    pub fn new(
        dbt: &Rc<RefCell<DatabaseTable>>,
        buf: &Rc<RefCell<BufPageManager>>,
        path: &str,
    ) -> QueryRunner {
        QueryRunner {
            db_table: Rc::clone(dbt),
            buf_page: Rc::clone(buf),
            base_path: path.to_string(),
            db_ctx: Rc::new(RefCell::new(None)),
        }
    }

    fn get_meta(&self, name: &str) -> Result<TableMeta, Box<dyn std::error::Error>> {
        let ctx = self.db_ctx.try_borrow()?;
        if let Some(ctx) = &*ctx {
            ctx.read_table(name)
        } else {
            Err(Box::new(DBError::new("no database selected!")))
        }
    }
}
